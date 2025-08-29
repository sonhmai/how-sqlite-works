use crate::lib::candidate::Candidate;
use crate::lib::catalog::Catalog;
use crate::lib::query::{CmpOp, FilterPred, JoinPred};

#[derive(Debug, Eq, PartialEq)]
pub enum JoinAlgo {
    Hash,
    NestedLoop
}

/// A pluggable cost model for join algorithms.
/// Implementors can override the formulas to reflect their engine’s runtime.
/// All costs are in arbitrary units (relative scale is what matters).
pub trait JoinCostModel {
    /// Cost of a hash join (build + probe).
    fn hash_join(&self, left_rows: f64, right_rows: f64) -> f64;

    /// Cost of a nested-loop join (pairwise comparisons).
    fn nested_loop(&self, left_rows: f64, right_rows: f64) -> f64;

    /// Multiplier applied when performing a cross join (no join predicates).
    /// Engines usually penalize cross joins to avoid picking them accidentally.
    fn cross_penalty(&self) -> f64 {
        2.0
    }
}

/// A simple default cost model:
/// - HashJoin: O(left + right)
/// - NestedLoop: 0.5 * left * right
/// - Cross penalty: x2
#[derive(Clone, Debug, Default)]
pub struct DefaultJoinCostModel;

impl JoinCostModel for DefaultJoinCostModel {
    
    /// Hash Join
    /// - build phase: build smaller input (build side) to hash table `O(left_rows)`
    /// - probe phase: scan large input, for each tuple compute hash of join key,
    /// check the hash table -> `O(right_rows)`
    /// 
    /// -> total work O(left + right)
    /// 
    /// For production, it should consider also
    /// - memory usage: if hash table not fit in RAM, add IO spill cost (partition + multi-pass join)
    /// - parallelism: scaled down factor if we have parallelism
    fn hash_join(&self, left_rows: f64, right_rows: f64) -> f64 {
        (left_rows + right_rows)
    }

    /// Nested Loop Join: for each row in outer left table, the entire 
    /// right tables is scanned for matches -> `O(left_rows x right_rows)` time complexity.
    /// For example: 1,000 customers × 10 accounts = 10,000 comparisons.
    /// 
    /// 0.4 is used as a damping factor to take into account things that
    /// reduce the NLJ cost e.g. inner table caching.
    fn nested_loop(&self, left_rows: f64, right_rows: f64) -> f64 {
        0.4 * left_rows * right_rows
    }

    fn cross_penalty(&self) -> f64 {
        2.0
    }
}

/// Decide which join algorithm to use and return (algo, cost).
///
/// * `has_equi_preds` = true means we have usable equi-join keys.
///   If false, we treat it as a cross join and apply `cross_penalty()`.
pub fn choose_join<M: JoinCostModel>(
    model: &M,
    left_rows: f64,
    right_rows: f64,
    has_equi_preds: bool,
) -> (JoinAlgo, f64) {
    if has_equi_preds {
        let hash_cost = model.hash_join(left_rows, right_rows);
        let nested_loop_cost = model.nested_loop(left_rows, right_rows);
        if hash_cost <= nested_loop_cost {
            (JoinAlgo::Hash, hash_cost)
        } else {
            (JoinAlgo::NestedLoop, nested_loop_cost)
        }
    } else {
        let n = model.nested_loop(left_rows, right_rows) * model.cross_penalty();
        (JoinAlgo::NestedLoop, n)
    }
}

pub fn compose_join_candidate(
    mask: u64,
    l: &Candidate,
    r: &Candidate,
    crossing_preds: &[&JoinPred],
    algo: JoinAlgo,
    total_cost: f64,
    out_rows: f64,
) -> Candidate {
    let join_desc = if crossing_preds.is_empty() {
        "CROSS".to_string()
    } else {
        crossing_preds.iter()
            .map(|p| format!("{}.{} = {}.{}",
                             p.left.0, p.left.1, p.right.0, p.right.1))
            .collect::<Vec<_>>()
            .join(" AND ")
    };

    let plan = match algo {
        JoinAlgo::Hash =>
            format!("HashJoin({}) ⨝ [{}] ({})", l.plan, join_desc, r.plan),
        JoinAlgo::NestedLoop =>
            format!("NestedLoopJoin({}) ⨝ [{}] ({})", l.plan, join_desc, r.plan),
    };

    Candidate { mask, cost: total_cost, cardinality: out_rows, plan }
}

pub fn estimate_join_output(
    cat: &Catalog,
    l_card: f64,
    r_card: f64,
    crossing_preds: &[&JoinPred],
) -> f64 {
    let mut join_sel = 1.0;
    for jp in crossing_preds { join_sel *= join_selectivity(cat, jp); }
    join_sel = clamp01(join_sel);
    (l_card * r_card * join_sel).max(1.0)
}

/// Estimate fraction of rows that are still there after a filter
/// e.g. `WHERE city = 'Hanoi'`.
/// 
/// Example: 
/// 
/// Table: Customer, rows = 1,000,000. city has NDV = 500 (≈ 500 unique cities)
/// 
/// Query filter: Customer.city = 'Hanoi' `sel = 1.0 / 500 ≈ 0.002`
/// 
/// Estimated surviving rows = 1,000,000 * 0.002 = 2,000 customers in Hanoi.
pub fn filter_selectivity(cat: &Catalog, fp: &FilterPred) -> f64 {
    let ts = cat.get(&fp.table);
    let ndv = ts.ndv.get(&fp.col).cloned().unwrap_or((ts.rows).sqrt());
    match fp.op {
        CmpOp::Eq => 1.0 / ndv.max(1.0),       // equality filter
        CmpOp::Gt | CmpOp::Lt | CmpOp::Ge | CmpOp::Le => 1.0/3.0, // crude guess for range
    }
}

/// Estimate fraction of pairs that survive a join condition e.g.
/// `customer.customer_id = account.customer_id`.
/// Why?
/// - optimizer can prune bad join orders (avoid exploding intermediate results).
/// - pick join algos: 
///     - Nested Loop maybe cheaper if output is small
///     - Hash probably cheaper if output is large
/// 
/// Example
/// 
/// - Table Customer. rows = 1,000,000. customer_id NDV = 1,000,000
/// - Table Account. rows = 5,000,000. customer_id NDV = 1,000,000
/// - Join: Customer.customer_id = Account.customer_id
/// - Computation: NDV left = 1,000,000. NDV right = 1,000,000 
/// 
/// Selectivity = 1 / max(1,000,000, 1,000,000) = 1e-6
/// 
/// Row estimate
/// 1,000,000 * 5,000,000 * 1e-6 = 5,000,000 joined rows
/// (→ roughly 5 accounts per customer).
pub fn join_selectivity(catalog: &Catalog, join_pred: &JoinPred) -> f64 {
    let left_stats = catalog.get(&join_pred.left.0);
    let right_stats = catalog.get(&join_pred.right.0);
    let ndv_l = left_stats.ndv
        .get(&join_pred.left.1).cloned()
        .unwrap_or(left_stats.rows.sqrt());
    let ndv_r = right_stats.ndv
        .get(&join_pred.right.1).cloned()
        .unwrap_or(right_stats.rows.sqrt());
    
    clamp01(1.0 / ndv_l.max(ndv_r).max(1.0))
}

/// Ensure input probability, selectivity is in (0, 1] range.
/// Why? 
/// - In real query optimizers, selectivity formulas are heuristic and can overshoot or undershoot.
/// - Formulas (1/NDV, 1/3) could give weird results if NDV is mis-specified (like 0, or smaller than 1).
/// - A cardinality of 0 might cause optimizer to think a join is free and pick nonsense plans.
pub fn clamp01(x: f64) -> f64 {
    if x < 1e-12 { 1e-12 } else if x > 1.0 { 1.0 } else { x }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_hash_for_large_inputs_with_equi_preds() {
        let cost_model = DefaultJoinCostModel::default();
        let left_rows = 1_000_000.0;
        let right_rows = 800_000.0;
        let (algo, cost) = choose_join(&cost_model, left_rows, right_rows, true);
        assert_eq!(algo, JoinAlgo::Hash);
        assert!(cost <= cost_model.nested_loop(left_rows, right_rows));
    }

    #[test]
    fn prefers_nested_loop_for_tiny_inputs_with_equi_preds() {
        let cost_model = DefaultJoinCostModel::default();
        let left_rows = 2.0;
        let right_rows = 10.0;
        let (algo, cost) = choose_join(&cost_model, left_rows, right_rows, true);
        assert_eq!(algo, JoinAlgo::NestedLoop);
        assert!((cost - cost_model.nested_loop(left_rows, right_rows)).abs() < 1e-9);
    }
}
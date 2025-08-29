use std::collections::{HashMap, HashSet};
use crate::lib::candidate::Candidate;
use crate::lib::catalog::Catalog;
use crate::lib::join::{choose_join, clamp01, compose_join_candidate, estimate_join_output, DefaultJoinCostModel};
use crate::lib::query::{FilterPred, JoinPred, Query};

pub mod catalog;
pub mod table_stats;
pub mod query;
pub mod candidate;
pub mod join;
pub mod index;

/// System R Dynamic Programming over join order
pub fn optimize(query: &Query, cat: &Catalog) -> Candidate {
    let n = query.tables.len();
    assert!(n > 0 && n <= 20, "supports up to 20 tables via bitsets");

    let index_of = build_index(&query.tables);
    let filters_by_table = group_filters_by_table(&query.filters);
    let all_mask: u64 = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };

    // 1) Seed DP with single-table access paths
    let mut best = init_base_candidates(query, cat, &index_of, &filters_by_table);

    // 2) DP: grow subsets and keep cheapest plan per mask
    for size in 2..=n as u32 {
        for mask in enumerate_masks_of_size(all_mask, size) {
            if let Some(c) = best_partition_for_mask(mask, &best, query, cat) {
                best.insert(mask, c);
            }
        }
    }

    best.get(&all_mask).expect("no final plan?").clone()
}


fn build_index(tables: &[String]) -> HashMap<String, usize> {
    let mut index_of = HashMap::new();
    for (i, t) in tables.iter().enumerate() {
        index_of.insert(t.clone(), i);
    }
    index_of
}

fn group_filters_by_table(filters: &[FilterPred]) -> HashMap<String, Vec<FilterPred>> {
    let mut map: HashMap<String, Vec<FilterPred>> = HashMap::new();
    for f in filters {
        map.entry(f.table.clone()).or_default().push(f.clone());
    }
    map
}

fn init_base_candidates(
    query: &Query,
    cat: &Catalog,
    index_of: &HashMap<String, usize>,
    filters_by_table: &HashMap<String, Vec<FilterPred>>,
) -> HashMap<u64, Candidate> {
    let mut best: HashMap<u64, Candidate> = HashMap::new();

    for t in &query.tables {
        todo!()
    }

    best
}

fn enumerate_masks_of_size(all_mask: u64, size: u32) -> impl Iterator<Item=u64> {
    (1u64..=all_mask).filter(move |m| popcount(*m) == size)
}

fn popcount(x: u64) -> u32 { x.count_ones() }

fn tables_in_mask(mask: u64, tables: &[String]) -> HashSet<String> {
    let mut s = HashSet::new();
    for (i, t) in tables.iter().enumerate() {
        if (mask & (1u64 << i)) != 0 { s.insert(t.clone()); }
    }
    s
}

fn proper_subsets(mask: u64) -> impl Iterator<Item=u64> {
    // non-empty, proper subsets
    let mut v = Vec::new();
    let mut sub = (mask - 1) & mask;
    while sub > 0 {
        v.push(sub);
        sub = (sub - 1) & mask;
    }
    v.into_iter()
}

fn collect_crossing_preds<'a>(
    joins: &'a [JoinPred],
    left_tables: &HashSet<String>,
    right_tables: &HashSet<String>,
) -> Vec<&'a JoinPred> {
    let mut crossing = vec![];
    for jp in joins {
        let lside = left_tables.contains(&jp.left.0) && right_tables.contains(&jp.right.0);
        let rside = left_tables.contains(&jp.right.0) && right_tables.contains(&jp.left.0);
        if lside || rside {
            crossing.push(jp);
        }
    }
    crossing
}

fn best_partition_for_mask(
    mask: u64,
    best: &HashMap<u64, Candidate>,
    query: &Query,
    cat: &Catalog,
) -> Option<Candidate> {
    let left_rights = proper_subsets(mask).map(|left| (left, mask ^ left))
        .filter(|(_, right)| *right != 0);

    let left_tables_all = |m: u64| tables_in_mask(m, &query.tables);

    let mut best_here: Option<Candidate> = None;
    let model = DefaultJoinCostModel::default();

    for (left, right) in left_rights {
        let (l_cand, r_cand) = match (best.get(&left), best.get(&right)) {
            (Some(lc), Some(rc)) => (lc, rc),
            _ => continue,
        };

        let l_tables = left_tables_all(left);
        let r_tables = left_tables_all(right);

        let crossing_preds = collect_crossing_preds(&query.joins, &l_tables, &r_tables);
        let out_rows = estimate_join_output(cat, l_cand.cardinality, r_cand.cardinality, &crossing_preds);

        // NEW: call into the join_algo module
        let (algo, jcost) = choose_join(
            &model,
            l_cand.cardinality,
            r_cand.cardinality,
            !crossing_preds.is_empty(),
        );

        let total_cost = l_cand.cost + r_cand.cost + jcost;
        let cand = compose_join_candidate(mask, l_cand, r_cand, &crossing_preds, algo, total_cost, out_rows);

        if best_here.as_ref().map(|c| c.cost).unwrap_or(f64::INFINITY) > cand.cost {
            best_here = Some(cand);
        }
    }

    best_here
}
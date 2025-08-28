use std::collections::HashMap;
use crate::lib::catalog::Catalog;
use crate::lib::optimize;
use crate::lib::query::{CmpOp, FilterPred, JoinPred, Lit, Query};
use crate::lib::table_stats::TableStats;

mod lib;

fn main() {
    let mut catalog = Catalog::default();
    init_catalog_data(&mut catalog);
    let query = get_query();
    let best = optimize(&query, &catalog);
    println!("=== Best Plan ===");
    println!("{}", best.plan);
    println!("estimated rows: {:.2}", best.cardinality);
    println!("total cost:     {:.2}", best.cost);
}

fn get_query() -> Query {
    // Query: Customer ⋈ Account on customer_id, Account ⋈ Transaction on account_id
    // Filters: Customer.city = 'Hanoi', Transaction.amount > 1000
    return Query {
        tables: vec!["Customer".into(), "Account".into(), "Transaction".into()],
        filters: vec![
            FilterPred { table: "Customer".into(), col: "city".into(), op: CmpOp::Eq, lit: Lit::Str("Hanoi") },
            FilterPred { table: "Transaction".into(), col: "amount".into(), op: CmpOp::Gt, lit: Lit::Int(1000) },
        ],
        joins: vec![
            JoinPred { left: ("Customer".into(), "customer_id".into()), right: ("Account".into(), "customer_id".into()) },
            JoinPred { left: ("Account".into(), "account_id".into()), right: ("Transaction".into(), "account_id".into()) },
        ],
    };
}

fn init_catalog_data(catalog: &mut Catalog) {
    catalog.stats.insert("Customer".to_string(), TableStats {
        rows: 1_000_000.0,
        row_width: 120.0,
        ndv: HashMap::from([
            ("customer_id".into(), 1_000_000.0),
            ("age".into(), 80.0),
            ("city".into(), 500.0),
        ]),
    });

    // Account table
    catalog.stats.insert("Account".to_string(), TableStats {
        rows: 5_000_000.0,
        row_width: 100.0,
        ndv: HashMap::from([
            ("account_id".into(), 5_000_000.0),
            ("customer_id".into(), 1_000_000.0),
            ("balance".into(), 1_000_000.0),
        ]),
    });

    // Transaction table
    catalog.stats.insert("Transaction".to_string(), TableStats {
        rows: 50_000_000.0,
        row_width: 150.0,
        ndv: HashMap::from([
            ("txn_id".into(), 50_000_000.0),
            ("account_id".into(), 5_000_000.0),
            ("txn_date".into(), 365.0),
            ("amount".into(), 1_000_000.0),
        ]),
    });
}
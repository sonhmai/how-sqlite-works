use std::collections::HashMap;
use crate::lib::table_stats::TableStats;

#[derive(Debug, Default)]
/// Contains statistics for each table.
pub struct Catalog {
    pub(crate) stats: HashMap<String, TableStats>,
}

impl Catalog {
    pub(crate) fn get(&self, table_name: &str) -> &TableStats {
        self.stats.get(table_name).expect("missing table stats")
    }
}
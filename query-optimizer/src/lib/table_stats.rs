use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TableStats {
    /// Number of rows
    pub(crate) rows: f64,
    /// Bytes per row
    pub(crate) row_width: f64,
    /// Number of distinct values of each column
    pub(crate) ndv: HashMap<String, f64>,          // column -> NDV
}


#[derive(Clone, Copy, Debug)]
pub enum CmpOp { Eq, Gt, Lt, Ge, Le }

#[derive(Clone, Debug)]
pub enum Lit { Int(i64), Str(&'static str), Float(f64) }

#[derive(Clone, Debug)]
pub struct FilterPred {
    pub(crate) table: String,
    pub(crate) col: String,
    pub(crate) op: CmpOp,
    pub(crate) lit: Lit,
}

#[derive(Clone, Debug)]
pub struct JoinPred {
    pub(crate) left: (String, String),   // (table, col)
    pub(crate) right: (String, String),  // (table, col)
}

#[derive(Clone, Debug)]
pub struct Query {
    pub(crate) tables: Vec<String>,
    pub(crate) filters: Vec<FilterPred>,
    pub(crate) joins: Vec<JoinPred>,
}
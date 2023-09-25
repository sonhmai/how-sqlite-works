use crate::physical::expression::physical_expr::PhysicalExpr;

pub struct PhysicalColByIndex {
    // index of the column in the record values
    pub(crate) col_index: usize
}

impl PhysicalExpr for PhysicalColByIndex {

}
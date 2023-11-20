use std::sync::Arc;

use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;
use crate::physical::plan::exec::Exec;

/// Hash Join Physical Plan for equi joins
#[derive(Debug)]
pub struct ExecJoinHash {
    /// left (build) side which gets hashed
    pub left: Arc<dyn Exec>,
    /// right (probe) side which are filtered by hash table
    pub right: Arc<dyn Exec>,
    // Set of equijoin columns from the relations: (left_col, right_col)
    // pub on: Vec<(Column, Column)>,
    // OUTER, INNER, etc.
    // pub join_type: JoinType,

    // TODO output schema for the join
    // do we need a separate schema field if using the volcano processing
    // model row by row? How does Spark do it?
}

impl Exec for ExecJoinHash {
    fn execute(&mut self) -> Vec<DataRecord> {
        todo!()
    }
    
    fn schema(&self) -> arrow_schema::SchemaRef {
        todo!()
    }
}

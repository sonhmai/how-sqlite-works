use std::sync::Arc;

use anyhow::{Ok, Result, bail};
use arrow_schema::SchemaRef;
use datafusion_common::JoinType;
use datafusion_physical_plan::joins::utils::{build_join_schema, JoinOn};
use datafusion_physical_plan::expressions::Column;

use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;
use crate::physical::plan::exec::Exec;

/// Nested loop join physical plan.
/// Note that NestedLoop does not have on join condition as HashJoin.
#[derive(Debug)]
pub struct ExecJoinNestedLoop {
    /// left (build) side which gets hashed
    pub left: Arc<dyn Exec>,
    /// right (probe) side which are filtered by hash table
    pub right: Arc<dyn Exec>,
    /// the type of join: OUTER, INNER, etc.
    pub join_type: JoinType,
    /// Schema once the join is applied
    schema: SchemaRef,
}

impl ExecJoinNestedLoop {

    pub fn try_new(
        left: Arc<dyn Exec>, 
        right: Arc<dyn Exec>,
        join_type: &JoinType, // using reference as we read only
    ) -> Result<Self> {
        todo!()
    }
}

impl Exec for ExecJoinNestedLoop {
    fn execute(&mut self) -> &[DataRecord] {
        todo!()
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_table() -> Arc<dyn Exec> {
        todo!()
    }

    #[test]
    fn test_join_on_1_column_pair() {

    }
}
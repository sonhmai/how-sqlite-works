use std::sync::Arc;

use anyhow::{Ok, Result};
use arrow_schema::SchemaRef;
use datafusion_common::JoinType;
use datafusion_physical_plan::joins::utils::build_join_schema;

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
    /// the type of join: OUTER, INNER, etc.
    pub join_type: JoinType,

    // TODO output schema for the join
    // do we need a separate schema field if using the volcano processing
    // model row by row? How does Spark do it?
    /// Schema once the join is applied
    schema: SchemaRef,
}

impl ExecJoinHash {

    /// try_new takes a reference to a JoinType (&JoinType) instead of 
    /// taking ownership of it (JoinType) for a couple of reasons:
    /// 
    /// 1. Efficiency: Taking a reference is more efficient than taking ownership 
    /// if the function does not need to consume or modify the value. 
    /// This is because taking a reference does not involve copying or moving the value.
    /// 
    /// 2. Flexibility: By taking a reference, the try_new function allows the caller 
    /// to continue using the JoinType value after the call.
    /// 
    /// Then why used ownership (JoinType) in the struct?
    ///     1. Lifetime: if reference is used, lifetime of join_type must be ensured
    ///     to be dropped not before the struct.
    ///     2. SOLID: the struct does not need to depends on the caller to maintain
    ///     the reference and can function, be tested on its own.
    ///     3. Copying is cheap for JoinType enum -> simplify the code by not using
    ///     reference.
    pub fn try_new(
        left: Arc<dyn Exec>, 
        right: Arc<dyn Exec>,
        join_type: &JoinType, // using reference as we read only
    ) -> Result<Self> {
        let left_schema = left.schema();
        let right_schema = right.schema();
        let (schema, column_indices) = 
            build_join_schema(&left_schema, &right_schema, join_type);

        Ok(ExecJoinHash {
            left,
            right,
            // dereferences the join_type reference (*join_type) to copy value into struct
            join_type: *join_type,
            schema: Arc::new(schema),
        })
    }
}

impl Exec for ExecJoinHash {
    fn execute(&mut self) -> Vec<DataRecord> {
        todo!()
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

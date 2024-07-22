use std::sync::Arc;

use anyhow::{bail, Ok, Result};
use arrow_schema::SchemaRef;
use datafusion_common::JoinType;
use datafusion_physical_plan::expressions::Column;
use datafusion_physical_plan::joins::utils::{build_join_schema, JoinOn};

use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

/// Hash Join Physical Plan for equi joins
#[derive(Debug)]
pub struct ExecJoinHash {
    /// left (build) side which gets hashed
    pub left: Arc<dyn Exec>,
    /// right (probe) side which are filtered by hash table
    pub right: Arc<dyn Exec>,
    /// Set of equijoin columns from the relations: (left_col, right_col)
    pub on: Vec<(Column, Column)>,
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
        on: JoinOn,
        join_type: &JoinType, // using reference as we read only
    ) -> Result<Self> {
        if on.is_empty() {
            bail!("On constraints in ExecJoinHash should be non-empty")
        }

        let left_schema = left.schema();
        let right_schema = right.schema();
        let (schema, column_indices) = build_join_schema(&left_schema, &right_schema, join_type);

        Ok(ExecJoinHash {
            left,
            right,
            on,
            // dereferences the join_type reference (*join_type) to copy value into struct
            join_type: *join_type,
            schema: Arc::new(schema),
        })
    }
}

impl Exec for ExecJoinHash {
    fn execute(&mut self) -> &[DataRecord] {
        todo!()
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use arrow_schema::{Schema, DataType, Field};
    use log::info;

    use crate::physical::plan::scan::ExecMemTable;

    use super::*;

    fn build_table(
        a: (&str, &Vec<i32>),
        b: (&str, &Vec<i32>),
        c: (&str, &Vec<i32>),
    ) -> Arc<dyn Exec> {
        let schema = Schema::new(vec![
            Field::new(a.0, DataType::Int32, false),
            Field::new(b.0, DataType::Int32, false),
            Field::new(c.0, DataType::Int32, false),
        ]);
        let records = vec![];
        
        Arc::new(
            ExecMemTable::new(&records, Arc::new(schema))
        )
    }

    #[test]
    fn test_join_on_1_column_pair() -> Result<()> {
        // joining left and right on a1 = a2
        let left_physical = build_table(
            ("a1", &vec![10, 1, 1]),
            ("b1", &vec![20, 1, 1]),
            ("c1", &vec![30, 1, 1]), // not matched in join
        );
        let right_physical = build_table(
            ("a2", &vec![10, 2, 2]),
            ("b2", &vec![20, 2, 2]),
            ("c2", &vec![40, 2, 2]), // not matched in join
        );
        let on: Vec<(Column, Column)> = vec![(
            Column::new_with_schema("a1", &left_physical.schema())?,
            Column::new_with_schema("a2", &right_physical.schema())?,
        )];
        
        let hash_join = ExecJoinHash::try_new(
            left_physical, 
            right_physical, 
            on,
            &JoinType::Inner
        )?;

        info!("{:?}", hash_join.schema());

        Ok(())
    }
}

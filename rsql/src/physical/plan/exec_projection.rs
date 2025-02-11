use std::sync::Arc;

use anyhow::Result;

use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;
use crate::physical::plan::exec::Exec;

/// A projection determines which columns or expressions are returned from a query.
///
/// The SQL statement `SELECT a, b, a+b FROM t1` is an example of a projection on table t1.
/// 3 projections expressions
///     a
///     b
///     a + b
///
/// SELECT without FROM will only evaluate expressions.
#[derive(Debug)]
pub struct ExecProjection {
    /// Physical plan input into this Exec for example
    /// SourceScan, CsvScan, SqliteTableScan, etc.
    pub(crate) input: Arc<dyn Exec>,
    /// expressions to be projected on the returned row
    pub(crate) expressions: Vec<Arc<dyn PhysicalExpr>>,
    result: Vec<DataRecord>,
}

impl ExecProjection {
    pub fn new(input: Arc<dyn Exec>, expressions: Vec<Arc<dyn PhysicalExpr>>) -> Result<Self> {
        Ok(Self {
            input,
            expressions,
            result: vec![],
        })
    }

    // TODO project by column name
    fn project(&self, record: &DataRecord) -> DataRecord {
        let mut values: Vec<ColumnValue> = vec![];
        for expr in &self.expressions {
            values.push(expr.evaluate(record));
        }
        DataRecord {
            values,
            rowid: record.rowid,
        }
    }
}

impl Exec for ExecProjection {
    fn execute(&mut self) -> &[DataRecord] {
        // why use into_iter() here instead of iter()?
        // https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/
        // into_iter() yields any of T (moved value), &T or &mut T depending on the usage
        // seems like we need a moved value here, not sure why we need yet.
        // self.result = *(self.input)
        //     .execute()
        //     .into_iter()
        //     .map(|record| self.project(&record))
        //     .collect();

        // &self.result

        todo!()
    }

    fn schema(&self) -> arrow_schema::SchemaRef {
        todo!()
    }
}

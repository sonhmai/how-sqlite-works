use crate::model::data_record::DataRecord;
use crate::physical::exec::Exec;
use datafusion_expr::Expr;

/// A projection determines which columns or expressions are returned from a query.
///
/// The SQL statement `SELECT a, b, a+b FROM t1` is an example of a projection on table t1.
/// 3 projections expressions
///     a
///     b
///     a + b
///
/// SELECT without FROM will only evaluate expressions.
pub struct ExecProjection {
    // Physical plan input into this Exec for example
    // SourceScan, CsvScan, SqliteTableScan, etc.
    pub(crate) input: Box<dyn Exec>,
    // expressions to be projected on the returned row
    pub(crate) expressions: Vec<Expr>,
}

impl Exec for ExecProjection {

    // TODO project by column name
    fn execute(&self) -> Vec<DataRecord> {
        // why use into_iter() here instead of iter()?
        // https://www.becomebetterprogrammer.com/rust-iter-vs-iter_mut-vs-into_iter/
        // into_iter() yields any of T (moved value), &T or &mut T depending on the usage
        // seems like we need a moved value here, not sure why we need yet.
        self.input
            .execute()
            .into_iter()
            .map(|record| return record)
            .collect()
    }
}

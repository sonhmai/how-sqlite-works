use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;

struct PhysicalCast;

impl PhysicalExpr for PhysicalCast {
    fn evaluate(&self, record: &DataRecord) -> ColumnValue {
        todo!()
    }
}

#[test]
fn cast_float_to_int() {

}

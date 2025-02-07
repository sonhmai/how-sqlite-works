use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;

#[derive(Debug)]
pub struct PhysicalLiteral {
    pub value: ColumnValue,
}

impl PhysicalExpr for PhysicalLiteral {
    fn evaluate(&self, _record: &DataRecord) -> ColumnValue {
        match &self.value {
            ColumnValue::Null => ColumnValue::Null,
            ColumnValue::One => ColumnValue::One,
            ColumnValue::Zero => ColumnValue::Zero,
            col => col.clone(),
        }
    }
}

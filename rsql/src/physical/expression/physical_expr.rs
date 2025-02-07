use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use std::fmt::Debug;

pub trait PhysicalExpr: Debug {
    // returns ColumnValue not &ColumnValue because we want the value to be copied
    // so it can be owned by others, not owned by the initial record
    fn evaluate(&self, record: &DataRecord) -> ColumnValue;
}

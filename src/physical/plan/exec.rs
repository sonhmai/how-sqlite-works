use crate::model::data_record::DataRecord;

pub trait Exec {
    // TODO use Iterator?
    fn execute(&self) -> Vec<DataRecord>;
}

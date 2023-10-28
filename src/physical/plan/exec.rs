use crate::model::data_record::DataRecord;
use std::fmt::Debug;

pub trait Exec: Debug {
    // TODO use Iterator?
    fn execute(&mut self) -> Vec<DataRecord>;
}

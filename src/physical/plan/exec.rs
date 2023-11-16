use crate::model::data_record::DataRecord;
use std::fmt::Debug;

/// Represent node in Physical Plan Tree
pub trait Exec: Debug {
    // TODO use Iterator?
    fn execute(&mut self) -> Vec<DataRecord>;

    // Get the schema for this Physical Plan
    // fn schema(&self) -> SchemaRef; // ??
}

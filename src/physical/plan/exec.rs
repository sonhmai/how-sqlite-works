use arrow_schema::SchemaRef;

use crate::model::data_record::DataRecord;
use std::fmt::Debug;

/// Represent node in Physical Plan Tree
pub trait Exec: Debug {
    // TODO use Iterator?
    fn execute(&mut self) -> Vec<DataRecord>;

    // Get the schema for this Physical Plan. Currenyly using arrow Schema.
    // Let's see later when project grows if depending on arrow for this is a good idea.
    fn schema(&self) -> SchemaRef;
}

use arrow_schema::SchemaRef;

use crate::model::data_record::DataRecord;
use std::fmt::Debug;

/// Represent node in Physical Plan Tree
pub trait Exec: Debug {
    // TODO use Iterator?
    /// Returns a slide to provide a read view without ownership.
    /// 
    /// execute can modify self, hence &mut. For example, recording 
    /// metrics in executing, changing self state.
    fn execute(&mut self) -> &[DataRecord];

    // Get the schema for this Physical Plan. Currenyly using arrow Schema.
    // Let's see later when project grows if depending on arrow for this is a good idea.
    fn schema(&self) -> SchemaRef;
}

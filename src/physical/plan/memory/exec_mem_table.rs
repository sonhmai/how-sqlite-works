use anyhow::Result;
use arrow_schema::SchemaRef;

use crate::physical::plan::exec::Exec;

/// An in-memory table scan for mocking data.
#[derive(Debug)]
pub struct ExecMemTable {
    schema: SchemaRef,
}

impl ExecMemTable {
    fn try_new() -> Result<Self> {
        todo!()
    }
}

impl Exec for ExecMemTable {
    fn execute(&mut self) -> Vec<crate::model::data_record::DataRecord> {
        todo!()
    }

    fn schema(&self) -> arrow_schema::SchemaRef {
        self.schema.clone()
    }
}

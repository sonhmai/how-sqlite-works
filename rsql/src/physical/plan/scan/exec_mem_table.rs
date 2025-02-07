use arrow_schema::SchemaRef;

use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

/// An in-memory table scan for mocking data.
#[derive(Debug)]
pub struct ExecMemTable {
    records: Vec<DataRecord>,
    schema: SchemaRef,
}

impl ExecMemTable {
    pub fn new(_records: &[DataRecord], _schema_ref: SchemaRef) -> Self {
        todo!()
    }
}

impl Exec for ExecMemTable {
    fn execute(&mut self) -> &[DataRecord] {
        &self.records
    }

    fn schema(&self) -> arrow_schema::SchemaRef {
        self.schema.clone()
    }
}

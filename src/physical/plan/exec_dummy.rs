use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

#[derive(Debug)]
pub struct ExecDummy {}

impl Exec for ExecDummy {
    fn execute(&mut self) -> Vec<DataRecord> {
        todo!()
    }
    
    fn schema(&self) -> arrow_schema::SchemaRef {
        todo!()
    }
}

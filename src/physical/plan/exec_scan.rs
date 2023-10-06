use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

#[derive(Debug)]
pub struct ExecScan {}

impl Exec for ExecScan {
    fn execute(&self) -> Vec<DataRecord> {
        todo!()
    }
}

use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

pub struct ExecScan {}

impl Exec for ExecScan {
    fn execute(&self) -> Vec<DataRecord> {
        todo!()
    }
}

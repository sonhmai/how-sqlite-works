use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

pub struct ExecDummy {}

impl Exec for ExecDummy {
    fn execute(&self) -> Vec<DataRecord> {
        todo!()
    }
}

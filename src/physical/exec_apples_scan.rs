use crate::model::data_record::DataRecord;
use crate::physical::exec::Exec;

///
/// A hard-coded scan physical operator for testing
/// TODO remove this after implementing proper sqlite Table scan
pub struct ExecApplesScan {}

impl Exec for ExecApplesScan {
    fn execute(&self) -> Vec<DataRecord> {
        vec![]
    }
}

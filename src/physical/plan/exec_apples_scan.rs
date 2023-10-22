use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

///
/// A hard-coded scan physical operator for testing
/// TODO remove this after implementing proper sqlite Table scan
#[derive(Debug)]
pub struct ExecApplesScan {}

impl Exec for ExecApplesScan {
    fn execute(&mut self) -> Vec<DataRecord> {
        vec![
            DataRecord {
                values: vec![
                    ColumnValue::int32(1),
                    ColumnValue::Text("Granny Smith".to_owned()),
                    ColumnValue::Text("Light Green".to_owned()),
                ],
                rowid: Some(1),
            },
            DataRecord {
                values: vec![
                    ColumnValue::int32(2),
                    ColumnValue::Text("Fuji".to_owned()),
                    ColumnValue::Text("Red".to_owned()),
                ],
                rowid: Some(2),
            },
            DataRecord {
                values: vec![
                    ColumnValue::int32(3),
                    ColumnValue::Text("Honeycrisp".to_owned()),
                    ColumnValue::Text("Blush Red".to_owned()),
                ],
                rowid: Some(3),
            },
            DataRecord {
                values: vec![
                    ColumnValue::int32(4),
                    ColumnValue::Text("Golden Delicious".to_owned()),
                    ColumnValue::Text("Yellow".to_owned()),
                ],
                rowid: Some(4),
            },
        ]
    }
}

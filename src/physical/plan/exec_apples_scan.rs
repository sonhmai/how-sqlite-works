use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::plan::exec::Exec;

///
/// A hard-coded scan physical operator for testing
/// TODO remove this after implementing proper sqlite Table scan
pub struct ExecApplesScan {}

impl Exec for ExecApplesScan {
    fn execute(&self) -> Vec<DataRecord> {
        vec![
            DataRecord {
                values: vec![
                    ColumnValue::Text(b"Granny Smith"),
                    ColumnValue::Text(b"Light Green"),
                ],
                rowid: Some(1),
            },
            DataRecord {
                values: vec![
                    ColumnValue::Text(b"Fuji"),
                    ColumnValue::Text(b"Red"),
                ],
                rowid: Some(2),
            },
            DataRecord {
                values: vec![
                    ColumnValue::Text(b"Honeycrisp"),
                    ColumnValue::Text(b"Blush Red"),
                ],
                rowid: Some(3),
            },
            DataRecord {
                values: vec![
                    ColumnValue::Text(b"Golden Delicious"),
                    ColumnValue::Text(b"Yellow"),
                ],
                rowid: Some(4),
            },
        ]
    }
}

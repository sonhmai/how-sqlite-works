use crate::model::column_value::ColumnValue;
use crate::model::data_record::DataRecord;
use crate::physical::expression::physical_expr::PhysicalExpr;

#[derive(Debug)]
pub struct PhysicalColByIndex {
    // index of the column in the record values
    pub(crate) col_index: usize,
}

impl PhysicalExpr for PhysicalColByIndex {
    fn evaluate(&self, record: &DataRecord) -> ColumnValue {
        // In Rust, every value has an owner, and there can only be one owner at a time.
        // When a value is assigned to a variable, the variable becomes the owner of the value.
        // If the value is moved to another variable, the original variable loses
        // ownership of the value. This model ensures that there are
        // no memory leaks or data races in Rust programs.

        // clone() method is a way to create a new instance of a value that
        // has the same data as the original value.

        // The new value will have a new owner, and the original value will retain its ownership.
        match record.value_at_index(self.col_index) {
            ColumnValue::Text(value) => ColumnValue::Text(value.clone()),
            ColumnValue::Blob(value) => ColumnValue::Blob(value.clone()),
            ColumnValue::Null => ColumnValue::Null,
            ColumnValue::One => ColumnValue::One,
            ColumnValue::Zero => ColumnValue::Zero,
            col => col.clone()
        }
    }
}

#[test]
fn test_col_index() {
    let data_record = DataRecord {
        values: vec![
            ColumnValue::Text("Granny Smith".to_owned()),
            ColumnValue::Text("Light Green".to_owned()),
            ColumnValue::Text("3".to_owned()),
        ],
        rowid: Some(1),
    };

    assert_eq!(
        PhysicalColByIndex { col_index: 1 }.evaluate(&data_record),
        ColumnValue::Text("Light Green".to_owned())
    );

    assert_eq!(
        PhysicalColByIndex { col_index: 0 }.evaluate(&data_record),
        ColumnValue::Text("Granny Smith".to_owned())
    );

    assert_eq!(
        PhysicalColByIndex { col_index: 2 }.evaluate(&data_record),
        ColumnValue::Text("3".to_owned())
    );
}
use crate::model::data_record::DataRecord;

/// show CLI output in SQLite format
pub fn sqlite_show(records: &[DataRecord]) {
    for record in records {
        let formatted_values: Vec<String> = record
            .values
            .iter()
            .map(|value| value.to_string())
            .collect();

        let output = formatted_values.join("|");
        println!("{}", output);
    }
}

use crate::model::column::ColumnValue;
use crate::varint::decode_varint;
use std::io::BufRead;

// https://github.com/bert2/build-your-own-sqlite-rust/blob/master/src/format/record.rs
// https://github.com/richardmarbach/codecrafters-sqlite-rust/blob/master/src/record.rs#L110
/// DataRecord needs a lifetime parameter 'a to tell the compiler that Vec values
/// has the same lifetime a as owning struct DataRecord
#[derive(Debug)]
pub struct DataRecord<'a> {
    pub values: Vec<ColumnValue<'a>>,
    pub num_header_bytes: u8,
    pub rowid: Option<u64>,
}

impl DataRecord<'_> {

    /// parse_from parses a DataRecord from a slice of bytes (database file byte stream)
    ///
    /// # Arguments
    ///
    /// * `payload` - a shared slice of bytes (u8) because we don't need to modify it.
    ///     A shared slice allows multiple reader access, but no writer.
    pub fn parse_from(rowid: u64, payload: &[u8], column_count: u8) -> DataRecord {
        let (header_size, mut header_offset) = decode_varint(payload);
        let mut content_offset = header_size as usize;
        let mut col_values = vec![];
        while header_offset < header_size as usize {
            let (serial_type, read_bytes) = decode_varint(&payload[header_offset..]);
            let (col_value, value_size) =
                ColumnValue::parse(serial_type, &payload[content_offset..]).unwrap();
            header_offset += read_bytes;
            content_offset += value_size;
        }
        DataRecord {
            values: col_values,
            rowid: Some(rowid),
            num_header_bytes: header_size.try_into().unwrap(),
        }
    }
}

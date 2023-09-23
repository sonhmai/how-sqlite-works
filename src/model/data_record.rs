use crate::model::column_value::ColumnValue;
use crate::varint::decode_varint;

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
    pub fn parse_from(rowid: u64, payload: &[u8]) -> DataRecord {
        let (header_size, mut header_offset) = decode_varint(payload);
        let mut content_offset = header_size as usize;
        let mut col_values = vec![];
        while header_offset < header_size as usize {
            let (serial_type, read_bytes) = decode_varint(&payload[header_offset..]);
            let (col_value, value_size) =
                ColumnValue::parse(serial_type, &payload[content_offset..]).unwrap();
            col_values.push(col_value);
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

#[test]
fn test_parse_record() {
    // CREATE TABLE t1(a,b,c);
    // INSERT INTO t1 VALUES(177, NULL, 'hello');
    //
    // 04 header size
    // 02 type 1
    // 00 type 2
    // 17 type 3
    // Data
    //  first 177 -> hex: 00B1 2 bytes
    //  second no bytes
    //  third "hello" -> hex: 68656C6F
    //
    // Column datatypes are optional
    // Datatypes are suggestions, not requirements
    let payload = hex::decode("0402001700B168656C6C6F").unwrap();
    let record = DataRecord::parse_from(1, &payload);
    println!("{record:?}");
    assert_eq!(record.num_header_bytes, 4);
    assert_eq!(record.rowid, Some(1));
    assert_eq!(record.values[0], ColumnValue::Int16([0, 177]));
    assert_eq!(record.values[1], ColumnValue::Null);
    assert_eq!(record.values[2], ColumnValue::Text(b"hello"));
}

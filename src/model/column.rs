///
/// https://www.sqlite.org/datatype3.html
/// Each value stored in an SQLite database (or manipulated by the database engine) has one of the following storage classes:
///   NULL. The value is a NULL value.
///   INTEGER. The value is a signed integer,
///     stored in 0, 1, 2, 3, 4, 6, or 8 bytes depending on the magnitude of the value.
///   REAL. The value is a floating point value, stored as an 8-byte IEEE
///     floating point number.
///   TEXT. The value is a text string, stored using the
///     database encoding (UTF-8, UTF-16BE or UTF-16LE).
///   BLOB. The value is a blob of data, stored exactly as it was input.
///
/// Any column in an SQLite version 3 database, except an INTEGER PRIMARY KEY column,
/// may be used to store a value of any storage class.
use anyhow::Result;

#[derive(Debug)]
enum ColumnType {
    Null,
    // integer
    I8,
    I16,
    I24,
    I32,
    I48,
    I64,
    // real
    F64,
    Text(usize),
    Blob(usize),
}

// https://github.com/richardmarbach/codecrafters-sqlite-rust/blob/master/src/record.rs#L110
// https://github.com/bert2/build-your-own-sqlite-rust/blob/master/src/format/col_content.rs
#[derive(Debug)]
pub enum ColumnValue {
    Null,
    Int8([u8; 1]),
    Int16([u8; 2]),
    Int24([u8; 3]),
    Int32([u8; 4]),
    Int48([u8; 6]),
    Int64([u8; 8]),
    Float64([u8; 8]),
    Zero,
    One,
    Blob([u8]),
    Text([u8]),
}

impl ColumnValue {
    pub fn parse(serial_type: i64, stream: &[u8]) -> Result<(ColumnValue, usize)> {
        Ok(match serial_type {
            0 => (ColumnValue::Null, 0),
            1 => (ColumnValue::Int8(stream[..1].try_into()?), 1),
            2 => (ColumnValue::Int16(stream[..2].try_into()?), 2),
            3 => (ColumnValue::Int24(stream[..3].try_into()?), 3),
            4 => (ColumnValue::Int32(stream[..4].try_into()?), 4),
            5 => (ColumnValue::Int48(stream[..6].try_into()?), 6),
            6 => (ColumnValue::Int64(stream[..8].try_into()?), 8),
            7 => (ColumnValue::Float64(stream[..8].try_into()?), 8),
            8 => (ColumnValue::Zero, 0),
            9 => (ColumnValue::One, 0),
            n if n >= 12 && n % 2 == 0 => {
                let len = ((n - 12) / 2).try_into()?;
                (ColumnValue::Blob(&stream[..len]), len)
            }
            n if n >= 13 && n % 2 == 1 => {
                let len = ((n - 13) / 2).try_into()?;
                (ColumnValue::Text(&stream[..len]), len)
            }
            n => panic!("Invalid serial type: {}", n),
        })
    }
}
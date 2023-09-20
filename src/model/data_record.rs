use std::io::BufRead;

#[derive(Debug)]
pub struct DataRecord {
    serial_types: Vec<u8>,
    num_header_bytes: u8,
    rowid: Option<u64>,
}

impl DataRecord {
    pub fn from_bytes(bytes: Box<dyn BufRead>, column_count: u8) -> DataRecord {
        todo!()
    }
}

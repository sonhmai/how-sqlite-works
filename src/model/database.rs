use anyhow::Result;
use std::fs::File;
use std::io::Read;

const MAGIC_HEADER: [u8; 16] = *b"SQLite format 3\0";
const ROOT_PAGE_OFFSET: u8 = 100;
const NUM_CELLS_OFFSET: u8 = 3;

/// A sqlite3 database (1 db file)
///
/// Ref
///   https://github.com/richardmarbach/codecrafters-sqlite-rust/blob/master/src/database.rs
///   https://github.com/bert2/build-your-own-sqlite-rust/blob/master/src/format/db_header.rs
#[derive(Debug)]
pub struct Database {
    pub file_path: Option<String>,
    pub page_size: u16, // 2 bytes
    pub db_page_count: u32,
}

impl Database {
    /// create a Database instance from file path
    pub fn new(file_path: &str) -> Result<Self> {
        println!("Creating Database from {file_path}");
        let mut file = File::open(file_path)?;
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf)?;

        Database::parse(&buf)
    }

    pub fn parse(stream: &[u8]) -> Result<Self> {
        Ok(Self {
            file_path: None,
            page_size: u16::from_be_bytes(stream[16..18].try_into()?),
            db_page_count: u32::from_be_bytes(stream[28..32].try_into()?),
        })
    }
}

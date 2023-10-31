use std::fs;
use std::path::PathBuf;

pub fn db_bytes() -> Vec<u8> {
    // CARGO_MANIFEST_DIR is project root /../rust-sqlite
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
    let data = fs::read(db_path).unwrap();
    data
}
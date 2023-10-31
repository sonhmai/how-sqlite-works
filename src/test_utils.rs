use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::storage::disk_manager::{DiskManager, SharedDiskManager};

pub fn db_bytes() -> Vec<u8> {
    // CARGO_MANIFEST_DIR is project root /../rust-sqlite
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
    let data = fs::read(db_path).unwrap();
    data
}

pub fn ref_disk_manager() -> SharedDiskManager {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
    let disk_manager = DiskManager::new(db_path.to_str().unwrap(), 4096).unwrap();
    let dm_ref = Rc::new(RefCell::new(disk_manager));
    dm_ref.clone()
}


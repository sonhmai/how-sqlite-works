use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::storage::default::DefaultDiskManager;
use crate::storage::disk_manager::SharedDiskManager;

use std::sync::Once;
static INIT: Once = Once::new();

pub fn file_bytes_vec(path_from_proj_root: &str) -> Vec<u8> {
    // CARGO_MANIFEST_DIR is project root /../rust-sqlite
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path_from_proj_root);
    let data = fs::read(db_path).unwrap();
    data
}

pub fn db_bytes() -> Vec<u8> {
    // CARGO_MANIFEST_DIR is project root /../rust-sqlite
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
    let data = fs::read(db_path).unwrap();
    data
}

pub fn ref_disk_manager() -> SharedDiskManager {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
    let disk_manager = DefaultDiskManager::new(db_path.to_str().unwrap(), 4096).unwrap();
    let dm_ref = Rc::new(RefCell::new(disk_manager));
    dm_ref.clone()
}

pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

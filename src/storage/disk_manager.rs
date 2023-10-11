use std::fs;
use std::path::PathBuf;

use crate::model::page::Page;
use crate::model::page_id::PageId;

/// Provides a logic abstraction for physical file on disk operations.
#[derive(Debug)]
pub struct DiskManager {
    pub db_file_path: String,
    pub page_size: usize,
}

impl DiskManager {
    pub fn new(db_file_path: &str, page_size: usize) -> anyhow::Result<Self> {
        Ok(Self {
            db_file_path: db_file_path.to_owned(),
            page_size,
        })
    }

    pub fn read_page(&self, page_id: PageId) -> anyhow::Result<Page> {
        let db = self.db_bytes();
        Page::parse(page_id.page_number, self.page_size, db.as_slice())
    }

    fn db_bytes(&self) -> Vec<u8> {
        // CARGO_MANIFEST_DIR is project root /../rust-sqlite
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(&self.db_file_path);
        // TODO read only the needed page instead of the whole thing into mem
        let data = fs::read(db_path).unwrap();
        data
    }
}
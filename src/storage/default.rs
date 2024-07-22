use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::fs;
use crate::model::page::Page;
use crate::model::page_id::PageId;
use crate::storage::disk_manager::DiskManager;

/// Provides a logic abstraction for physical file on disk operations.
#[derive(Debug)]
pub struct DefaultDiskManager {
    pub db_file_path: String,
    pub page_size: usize,
    num_writes: u32, // keeping track of number of writes to disk
}

impl DefaultDiskManager {
    pub fn new(db_file_path: &str, page_size: usize) -> anyhow::Result<Self> {
        Ok(Self {
            db_file_path: db_file_path.to_owned(),
            page_size,
            num_writes: 0,
        })
    }

    fn db_bytes(&self) -> Vec<u8> {
        // CARGO_MANIFEST_DIR is project root /../rust-sqlite
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&self.db_file_path);
        // TODO read only the needed page instead of the whole thing into mem
        
        fs::read(db_path).unwrap()
    }
}

impl DiskManager for DefaultDiskManager {
    /// Read a file from the database file.
    fn read_page(&self, page_id: PageId) -> anyhow::Result<Page> {
        let db = self.db_bytes();
        Page::parse(page_id.page_number, self.page_size, db.as_slice())
    }

    /// Write a file to the database file.
    fn write_page(&mut self, page_id: PageId, page: &Page) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().write(true).open(&self.db_file_path)?;

        let position = SeekFrom::Start((page_id.page_number * self.page_size as u32) as u64);
        file.seek(position)?;
        file.write_all(&page.data)?;
        self.num_writes += 1;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::page_id::PageId;
    use crate::storage::disk_manager::DiskManager;
    use std::path::PathBuf;
    use crate::storage::default::DefaultDiskManager;

    #[test]
    fn test_read_page() {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
        let dm = DefaultDiskManager::new(db_path.to_str().unwrap(), 4096).unwrap();
        let page_id = PageId { page_number: 2 };
        let page = dm.read_page(page_id).unwrap();

        assert_eq!(page.page_header.is_leaf(), true);
        assert_eq!(page.page_id.page_number, 2);
        assert_eq!(page.page_header.number_of_cells, 4);
    }
}

use anyhow::Result;

use crate::model::page_header::{PageHeader, PageType};
use crate::model::page_id::PageId;

/// A page in the SQLite database.
/// https://www.sqlite.org/fileformat.html#pages
///
/// Pages are numbered starting with 1 (there is no page 0).
/// The maximum page number is 4294967294 (2^32 - 2).
/// The minimum size SQLite database is a single 512-byte page.
#[derive(Debug)]
pub struct Page {
    pub page_header: PageHeader,
    pub page_id: PageId,
}

/// The first page is special because it contains
/// the sqlite_schema table.
#[derive(Debug)]
pub struct FirstPageSchema {
    pub page_header: PageHeader,
}

impl Page {
    /// page_number: SQLite 1-indexed page number starting with 1
    fn parse(page_number: u32, page_size: usize, db: &[u8]) -> Result<Self> {
        assert!(page_number >= 1);
        // page number in sqlite is 1-indexed (starts from 1, not 0)
        let page_offset = usize::try_from(page_number - 1)? * page_size;

        Ok(Page {
            page_header: PageHeader::parse(&db[page_offset..])?,
            page_id: PageId { page_number },
        })
    }

    // Return the id of this page.
    // The id is a unique identifier for a page that can be used to
    // - look up the page on disk
    // - or determine if the page exists in the buffer pool.
    fn get_id(&self) -> PageId {
        self.page_id
    }

    fn get_page_number(&self) -> u32 {
        self.page_id.page_number
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_table_leaf_page() {
        // CARGO_MANIFEST_DIR is project root /../rust-sqlite
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let data = fs::read(db_path).unwrap();
        let db_slice = data.as_slice();
        let first_data_page_number = 2;
        let page = Page::parse(first_data_page_number, 4096, db_slice).unwrap();

        assert_eq!(page.get_page_number(), 2);
        assert_eq!(page.page_header.page_type, PageType::LeafTable);
        assert_eq!(page.page_header.number_of_cells, 4);
    }
}

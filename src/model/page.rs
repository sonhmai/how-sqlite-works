use anyhow::Result;
use log::debug;

use crate::model::db_header::DbHeader;
use crate::model::page_header::{PageHeader, PageType};
use crate::model::page_id::PageId;

/// A page in the SQLite database.
/// https://www.sqlite.org/fileformat.html#pages
///
/// Pages are numbered starting with 1 (there is no page 0).
/// The maximum page number is 4294967294 (2^32 - 2).
/// The minimum size SQLite database is a single 512-byte page.
///
/// The first page is a special one because is has extra 100-byte
/// for the db header. So we need to aware whether it it is the first
/// db page or not to account for that.
#[derive(Debug)]
pub struct Page {
    pub page_header: PageHeader,
    pub page_id: PageId,
    pub data: Vec<u8>, // bytes of the page
}

impl Page {
    const SCHEMA_PAGE_NUM: u32 = 1;

    /// page_number: SQLite 1-indexed page number starting with 1
    pub fn parse(page_number: u32, page_size: usize, db: &[u8]) -> Result<Self> {
        assert!(page_number >= 2);
        // page number in sqlite is 1-indexed (starts from 1, not 0)
        let page_offset = usize::try_from(page_number - 1)? * page_size;

        Ok(Page {
            page_header: PageHeader::parse(&db[page_offset..])?,
            page_id: PageId { page_number },
            data: db[page_offset..page_offset + page_size].to_owned(),
        })
    }

    // Parse the first page of db file which is also the dn schema page.
    pub fn parse_db_schema_page(db: &[u8], page_size: usize) -> Result<Self> {
        // first page data starts after the DBHeader 100 bytes
        // https://www.sqlite.org/fileformat.html#b_tree_pages
        let page_offset = DbHeader::SIZE;

        Ok(Self {
            page_header: PageHeader::parse(&db[page_offset..])?,
            page_id: PageId { page_number: 1 },
            data: db[..page_size].to_owned(),
        })
    }

    // Return cell pointers of this page
    pub fn cell_ptrs(&self) -> Vec<usize> {
        // must offset extra Db header size if it's the first page
        // first page: DbHeader | PageHeader | CellPointers | ...
        // other page: PageHeader | CellPointers | ...
        let cell_ptrs_offset = self.page_header.size() +
            if self.is_db_schema_page() { DbHeader::SIZE } else { 0 };
        let num_cells: usize = self.page_header.number_of_cells.into();

        debug!("cell ptrs offset {cell_ptrs_offset}, num {num_cells}");

        self.data[cell_ptrs_offset..]
            .chunks_exact(2)
            .take(num_cells)
            .map(|two_bytes| usize::from(u16::from_be_bytes(two_bytes.try_into().unwrap())))
            .collect()
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

    fn is_db_schema_page(&self) -> bool {
        self.page_id.page_number == Page::SCHEMA_PAGE_NUM
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::*;

    const SAMPLE_DB_PAGE_SIZE: usize = 4096;

    fn db_bytes() -> Vec<u8> {
        // CARGO_MANIFEST_DIR is project root /../rust-sqlite
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let data = fs::read(db_path).unwrap();
        data
    }

    #[test]
    fn test_parse_table_leaf_page() {
        let db = db_bytes();
        let first_data_page_number = 2;
        let page = Page::parse(first_data_page_number, SAMPLE_DB_PAGE_SIZE, db.as_slice()).unwrap();

        // expected header: PageHeader {
        //  page_type: LeafTable, first_free_block_start: 0,
        //  number_of_cells: 4, content_start_offset: 4001, fragmented_free_bytes: 0,
        //  right_child_page_number: None
        // }
        assert_eq!(page.get_page_number(), 2);
        assert_eq!(page.page_header.page_type, PageType::LeafTable);
        assert_eq!(page.page_header.number_of_cells, 4);
        assert_eq!(page.page_header.right_child_page_number, None); // leaf page
        let cell_ptrs = page.cell_ptrs();
        assert_eq!(cell_ptrs.len(), 4);
        assert_eq!(cell_ptrs, vec![4067, 4054, 4029, 4001]);

    }

    #[test]
    fn test_parse_first_page() {
        let db = db_bytes();
        let page = Page::parse_db_schema_page(db.as_slice(), SAMPLE_DB_PAGE_SIZE).unwrap();

        assert_eq!(page.get_page_number(), 1);
        assert_eq!(page.is_db_schema_page(), true);
        assert_eq!(page.page_header.page_type, PageType::LeafTable);
        assert_eq!(page.page_header.number_of_cells, 3);
        let cell_ptrs = page.cell_ptrs();
        assert_eq!(cell_ptrs.len(), 3);
        assert_eq!(cell_ptrs, vec![3983, 3901, 3779]);
        // uncomment to get the specific bytes or the cell
        // println!("{:?}", &db[3983..3901]);
    }
}

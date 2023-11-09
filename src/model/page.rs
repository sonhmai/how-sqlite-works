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
    pub data: Vec<u8>,             // bytes of the page
    cell_ptrs: Option<Vec<usize>>, // pointers to data cell of this page
}

impl Page {
    /// Create a dummy page usually for mocking and testing.
    pub fn dummy() -> Self {
        Page {
            page_header: PageHeader::dummy(),
            page_id: PageId::new(999),
            data: vec![],
            cell_ptrs: None,
        }
    }
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
            cell_ptrs: None,
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
            cell_ptrs: None,
        })
    }

    /// Returns pointer to where cell bytes start in page's bytes of index-th cell.
    /// For example: a page has 4096 bytes, the cell we want located at bytes[2000,2010],
    /// this function returns 2000.
    ///
    /// If cell pointers of a page are [4067, 4054, 4029, 4001], cell_ptr an index 0 is 4067.
    /// First cell is at the end of a sqlite page, then it grows towards the beginning.
    pub fn get_cell_ptr(&mut self, index: usize) -> usize {
        let cell_ptrs = self.cell_ptrs();
        *cell_ptrs.get(index).unwrap()
    }

    /// Return cell pointers of this page
    pub fn cell_ptrs(&mut self) -> &Vec<usize> {
        if self.cell_ptrs.is_none() {
            self.parse_cell_ptrs();
        }
        // &self.cell_ptrs.unwrap() is not correct: trying to return a reference to a value that is owned by the current function
        // unwrap method returns a temporary value owned by this func.
        // -> solution: return a reference to the Vec<usize> inside the Option with as_ref on Option
        self.cell_ptrs.as_ref().unwrap()
    }

    /// Return whether page is a leaf page (table or index)
    pub fn is_leaf(&self) -> bool {
        self.page_header.is_leaf()
    }

    /// Return whether page is a leaf page (table or index)
    pub fn is_interior(&self) -> bool {
        self.page_header.is_interior()
    }

    fn parse_cell_ptrs(&mut self) {
        // must offset extra Db header size if it's the first page
        // first page: DbHeader | PageHeader | CellPointers | ...
        // other page: PageHeader | CellPointers | ...
        let cell_ptrs_offset = self.page_header.size()
            + if self.is_db_schema_page() {
                DbHeader::SIZE
            } else {
                0
            };
        let num_cells: usize = self.page_header.number_of_cells.into();

        debug!("cell ptrs offset {cell_ptrs_offset}, num {num_cells}");

        let ptrs: Vec<usize> = self.data[cell_ptrs_offset..]
            .chunks_exact(2)
            .take(num_cells)
            .map(|two_bytes| usize::from(u16::from_be_bytes(two_bytes.try_into().unwrap())))
            .collect();

        self.cell_ptrs = Some(ptrs);
    }

    pub fn get_number_of_cells(&self) -> u16 {
        self.page_header.number_of_cells
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
    use crate::test_utils::db_bytes;

    use super::*;

    const SAMPLE_DB_PAGE_SIZE: usize = 4096;

    #[test]
    fn test_parse_table_leaf_page() {
        let db = db_bytes();
        let first_data_page_number = 2;
        let mut page =
            Page::parse(first_data_page_number, SAMPLE_DB_PAGE_SIZE, db.as_slice()).unwrap();

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
        assert_eq!(*cell_ptrs, vec![4067, 4054, 4029, 4001]);
        assert_eq!(page.get_cell_ptr(0), 4067);
        assert_eq!(page.get_cell_ptr(1), 4054);
        assert_eq!(page.get_cell_ptr(2), 4029);
        assert_eq!(page.get_cell_ptr(3), 4001);
    }

    #[test]
    fn test_parse_first_page() {
        let db = db_bytes();
        let mut page = Page::parse_db_schema_page(db.as_slice(), SAMPLE_DB_PAGE_SIZE).unwrap();

        assert_eq!(page.get_page_number(), 1);
        assert_eq!(page.is_db_schema_page(), true);
        assert_eq!(page.page_header.page_type, PageType::LeafTable);
        assert_eq!(page.page_header.number_of_cells, 3);
        let cell_ptrs = page.cell_ptrs();
        assert_eq!(cell_ptrs.len(), 3);
        assert_eq!(*cell_ptrs, vec![3983, 3901, 3779]);
        // uncomment to get the specific bytes or the cell
        // println!("{:?}", &db[3983..3901]);
    }
}

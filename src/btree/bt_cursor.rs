use std::cell::RefCell;
use std::rc::Rc;
use anyhow::Result;

use crate::model::cell_table_interior::CellTableInterior;
use crate::model::cell_table_leaf::LeafTableCell;
use crate::model::data_record::DataRecord;

use crate::model::database::Database;
use crate::model::page::Page;
use crate::model::page_id::PageId;

#[derive(Debug)]
pub struct BtCursor {
    // Rc for multiple references to same object
    // RefCell allows mutable borrowing because we would want to modify contained obj
    database: Rc<RefCell<Database>>,
    page: Option<Rc<RefCell<Page>>>, // current page. ~ sqlite pCursor->pPage
    root_page_number: u32, // root page number of the btree
    index_current_page: u16,
}

impl BtCursor {
    pub fn new(database: Rc<RefCell<Database>>, root_page_number: u32) -> Self {
        BtCursor {
            database,
            page: None,
            root_page_number,
            index_current_page: 0
        }
    }

    pub fn scan_page(&mut self) -> TableScanIterator {
        let page_id = PageId::new(self.root_page_number);
        TableScanIterator { 
            database: self.database.clone(), 
            current_page_id: Some(page_id), 
            index: 0 
        }
    }

    pub fn move_to_next(&mut self) -> Option<DataRecord> {
        todo!()
    }

    pub fn move_to_previous(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the previous cell
        todo!()
    }

    pub fn move_to_last(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the last cell in the current page
        todo!()
    }

    pub fn move_to_first(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the first cell in the current page
        todo!()
    }

    /// The left-most leaf is the one with the smallest key -
    /// the first in ascending order.
    fn move_to_left_most_leaf(&mut self) {

    }
}

pub struct TableScanIterator {
    database: Rc<RefCell<Database>>,
    current_page_id: Option<PageId>,
    index: usize,
}

impl Iterator for TableScanIterator {
    type Item = LeafTableCell;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_page_id {
                Some(page_id) => {
                    let buffer_pool = &mut self.database.borrow_mut().buffer_pool;
                    let page = buffer_pool.get_page(page_id);
                    let mut page = page.borrow_mut();

                    if self.index < page.page_header.number_of_cells as usize {
                        let cell_ptr = page.get_cell_ptr(self.index);
                        self.index += 1;
                        match LeafTableCell::parse(&page.data[cell_ptr..]) {
                            Ok(cell) => return Some(cell),
                            // TODO error handling for parsing failed
                            Err(_) => return None, // If parsing fails, end the iteration
                        }
                    } else {
                        // Move to the next page
                        self.current_page_id = page.page_header.right_child_page_number.map(PageId::new);
                        self.index = 0;
                    }
                },
                None => return None, // No more pages
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_scan() {
        // superheroes.db has table spanning > 1 page
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/superheroes.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();
        let db_ref = Rc::new(RefCell::new(db));
        let mut cursor = BtCursor::new(db_ref.clone(), 2);

        assert_eq!(cursor.root_page_number, 2);
        assert!(cursor.page.is_none());

        let table_scan_iter = cursor.scan_page();
        for cell in table_scan_iter {
            println!("{cell:?}");
        }
    }

    #[test]
    fn test_move_to_next() {
    }

    #[test]
    fn test_move_to_previous() {
    }

    #[test]
    fn test_move_to_last() {
    }

    #[test]
    fn test_move_to_first() {
    }
}






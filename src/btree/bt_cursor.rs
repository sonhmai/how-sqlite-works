use std::cell::RefCell;
use std::rc::Rc;
use crate::model::data_record::DataRecord;

use crate::model::database::Database;
use crate::model::page::Page;
use crate::model::page_id::PageId;

#[derive(Debug)]
pub struct BtCursor {
    // Rc for multiple references to same object
    // RefCell allows mutable borrowing because we would want to modify contained obj
    database: Rc<RefCell<Database>>,
    page: Option<Rc<RefCell<Page>>>,
    root_page_number: u32, // root page number of the btree
    current_cell: i32,
    is_valid: bool,
}

impl BtCursor {
    pub fn new(database: Rc<RefCell<Database>>, root_page_number: u32) -> Self {
        BtCursor {
            database,
            page: None,
            root_page_number,
            current_cell: 0,
            is_valid: false,
        }
    }

    pub fn move_to_next(&mut self) -> Option<DataRecord> {
        todo!()
        // let buffer_pool = &mut self.database.borrow().buffer_pool;
        // // If the cursor is not valid, initialize it to the first cell of the root page
        // if !self.is_valid {
        //     let page_id =  PageId{page_number: self.root_page_number};
        //     let page = buffer_pool.get_page(page_id);
        //     self.page = Some(page);
        //     self.is_valid = true;
        // }
        //
        // loop {
        //     // If there's no current page, we've exhausted all pages
        //     let page = match &self.page {
        //         Some(page) => page.borrow(),
        //         None => return None,
        //     };
        //
        //     // If the current cell is a valid index in the page's cells, return the corresponding record
        //     if self.current_cell < page.get_number_of_cells() as i32 {
        //         let record = page.get_cell(self.current_cell).get_data_record();
        //         self.current_cell += 1;
        //         return Some(record);
        //     }
        //
        //     // If the current cell is not a valid index, move to the next page
        //     let page_id =  PageId{page_number: self.root_page_number};
        //     let page = buffer_pool.get_page(page_id);
        //     self.page = Some(page);
        //     self.current_cell = 0;
        // }
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
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_new() {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();
        let btree = Rc::new(RefCell::new(db));
        let cursor = BtCursor::new(btree.clone(), 1);

        assert_eq!(cursor.root_page_number, 1);
        assert_eq!(cursor.current_cell, 0);
        assert_eq!(cursor.is_valid, false);
        assert!(cursor.page.is_none());
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






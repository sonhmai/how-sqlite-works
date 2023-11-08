use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;

use crate::model::cell_table_leaf::LeafTableCell;
use crate::model::database::Database;
use crate::model::page::Page;
use crate::model::page_id::PageId;

// reference to a page in memory which is managed by BufferPool
type PageRef = Rc<RefCell<Page>>;

#[derive(Debug)]
pub struct BtCursor {
    // Rc for multiple references to same object
    // RefCell allows mutable borrowing because we would want to modify contained obj
    database: Rc<RefCell<Database>>,
    // current page cursor is pointing to. ~ sqlite pCursor->pPage
    page: Rc<RefCell<Page>>,
    // root page number of the btree
    root_page_number: u32,
    // index of current cell in current page that cursor is pointing to
    index_current_cell: u16,
    // index of current page in page stack
    index_current_page: u16,
    // stack of pages to current as we traverse down from the root
    page_stack: Vec<PageRef>,
}

impl BtCursor {
    pub fn new(database: Rc<RefCell<Database>>, root_page_number: u32) -> Self {
        let page_id = PageId::new(root_page_number);
        let page = database
            .borrow_mut()
            .buffer_pool
            .get_page(page_id);

        BtCursor {
            database,
            page,
            root_page_number,
            index_current_cell: 0,
            index_current_page: 0,
            page_stack: vec![],
        }
    }

    pub fn scan_page(&mut self) -> TableScanIterator {
        let page_id = PageId::new(self.root_page_number);
        TableScanIterator {
            database: self.database.clone(),
            current_page_id: Some(page_id),
            index: 0,
        }
    }

    fn page_ref(&mut self) -> Rc<RefCell<Page>> {
        self.page.clone()
    }

    /// Advance cursor to next entry in btree.
    pub fn move_to_next(&mut self) -> Result<()> {
        let page = self.page_ref();

        // if cursor is at the last cell of current page, go to next page
        let index_next_cell = self.index_current_cell + 1;
        if index_next_cell >= page.borrow().get_number_of_cells() {
            return self.next_entry_next_page();
        }

        // we know that we are not at the last cell of page, so cursor can advance.
        self.index_current_cell = index_next_cell;
        // if page is an interior, we want to move to left most leaf
        // so that cursor can point to the next entry.
        return if page.borrow().is_leaf() {
            Ok(())
        } else {
            self.move_to_left_most_leaf()
        };
    }

    /// Move to the next entry in the next page in case the cursor
    /// is currently at the last cell of current page (not possible to
    /// advance cell counter in current page).
    fn next_entry_next_page(&mut self) -> Result<()> {
        self.index_current_cell += 1;
        let current_index = self.index_current_cell;

        // TODO handle case page leaf/ interior
        let page = self.page_ref();
        if page.borrow().is_interior() {}

        Ok(())
    }

    pub fn move_to_previous(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the previous cell
        todo!()
    }

    /// Move cursor to last entry in the table.
    pub fn move_to_last(&mut self) -> Result<()> {
        todo!()
    }

    pub fn move_to_first(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the first cell in the current page
        todo!()
    }

    /// Move cursor to left-most leaf entry one level beneath currency entry
    /// the cursor is pointing to.
    ///
    /// The left-most leaf is the one with the smallest key -
    /// the first in ascending order.
    ///
    /// Equivalent to sqlite `static int moveToLeftmost(BtCursor *pCur)`
    fn move_to_left_most_leaf(&mut self) -> Result<()> {
        Ok(())
    }

    /// Move cursor to root page of its BTree.
    fn move_to_root(&mut self) -> Result<()> {
        // checks if the cursor is already at the root page (pCur->iPage >= 0).
        // If yes, release any pages that the cursor may have descended into
        // and returns to the root page.
        Ok(())
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
                        self.current_page_id =
                            page.page_header.right_child_page_number.map(PageId::new);
                        self.index = 0;
                    }
                }
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

    fn db_ref() -> Rc<RefCell<Database>> {
        // superheroes.db has table spanning > 1 page
        let db_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/superheroes.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();
        let db_ref = Rc::new(RefCell::new(db));
        db_ref
    }

    #[test]
    fn test_scan() {
        let mut cursor = BtCursor::new(db_ref().clone(), 2);

        assert_eq!(cursor.root_page_number, 2);

        let table_scan_iter = cursor.scan_page();
        for cell in table_scan_iter {
            println!("{cell:?}");
        }
    }

    #[ignore]
    #[test]
    fn test_move_to_root() {
        // should has no problem if cursor already pointed to root page
        let mut cursor = BtCursor::new(db_ref().clone(), 0);
        assert_eq!(cursor.root_page_number, 0);

        // should work when cursor moved away from root page
        let mut cursor = BtCursor::new(db_ref().clone(), 2);
        assert_eq!(cursor.root_page_number, 2);
        cursor.move_to_last().unwrap();
        assert_eq!(cursor.root_page_number, 2);
    }

    #[test]
    fn test_move_to_next() {}

    #[test]
    fn test_move_to_previous() {}

    #[test]
    fn test_move_to_last() {}

    #[test]
    fn test_move_to_first() {}
}

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{bail, Result};
use log::info;

use crate::model::cell_table_leaf::LeafTableCell;
use crate::model::database::Database;
use crate::model::page::Page;
use crate::model::page_id::PageId;

// reference to a page in memory which is managed by BufferPool
type PageRef = Rc<RefCell<Page>>;

pub struct BtCursor {
    /// Rc for multiple references to same object
    /// RefCell allows mutable borrowing because we would want to modify contained obj
    database: Rc<RefCell<Database>>,
    /// current page cursor is pointing to. ~ sqlite pCursor->pPage
    page: Rc<RefCell<Page>>,
    /// root page number of the btree
    root_page_number: u32,
    /// index of current cell in current page that cursor is pointing to
    index_current_cell: u16,

    /// Index of current page in page stack, increased when the cursor move_to_child
    /// (one level down the tree).
    index_current_page: u16,

    /// Stack of pages to current as we traverse down from the root.
    /// If the tree has 3 level and the cursor is at level 3 (index 2) the page_stack is
    /// - page_stack(2) = ptr to leaf page at level 3
    /// - page_stack(1) = ptr to interior page at level 2
    /// - page_stack(0) = ptr to interior page at level 1 (root)
    page_stack: Vec<PageRef>,

    /// Array of u16 current cell indices that cursor is accessing.
    /// Equivalent to `pCur->aiIdx` in sqlite source, type `u16 aiIdx[BTCURSOR_MAX_DEPTH-1]`.
    /// `&cell_index_stack[1]` is the cell index of page at level 1 in page_stack `&page_stack[1]`
    /// that the cursor is/ was accessing.
    cell_index_stack: Vec<u16>,
}

/// Not including some not-needed fields in Debug str (database, page, etc.)
/// to avoid distraction debug info.
impl std::fmt::Debug for BtCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BtCursor")
            .field("root_page_number", &self.root_page_number)
            .field("index_current_cell", &self.index_current_cell)
            .field("index_current_page", &self.index_current_page)
            .field("page_stack", &self.page_stack)
            .field("cell_index_stack", &self.cell_index_stack)
            .finish()
    }
}

impl BtCursor {
    /// root_page_number is 0-indexed. Db first page with db meta has page number 0.
    pub fn new(database: Rc<RefCell<Database>>, root_page_number: u32) -> Self {
        let page_id = PageId::new(root_page_number);
        let page = database.borrow_mut().buffer_pool.get_page(page_id);

        BtCursor {
            database,
            page,
            root_page_number,
            index_current_cell: 0,
            index_current_page: 0,
            page_stack: vec![],
            cell_index_stack: vec![],
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
        if page.borrow().is_leaf() {
            Ok(())
        } else {
            self.move_to_left_most_leaf_entry()
        }
    }

    /// Move to the next entry in the next page in case the cursor
    /// is currently at the last cell of current page (not possible to
    /// advance cell counter in current page).
    ///
    /// Similar to btreeNext(BtCursor *pCur) in sqlite source.
    fn next_entry_next_page(&mut self) -> Result<()> {
        /*
        steps
        - increment cursor current cell index to point to next cell
        - if index >= last cell (current page cell num)
            - leaf page -> cursor pass last entry in whole Btree
                -> INVALID cursor, somehow should signal caller we're done.
            - not leaf page (interior) -> move cursor to child page, then move to leftmost cell in child page.
        - else: case index within current page
            - current page is leaf
                - return Ok
            - move to leftmost cell in leaf child page beneath current. Why?
            current page is interior == cursor moved to cell that is pointer to another page.
            Remind that interior Btree has structure | Ptr0 | Key0 | Ptr1 | Key1 | ...
            Here it's not an data entry.
         */
        self.index_current_cell += 1;
        let current_index = self.index_current_cell;
        let page_rc = self.page_ref();

        let page = page_rc.borrow();

        if page.is_leaf() {
            if current_index < page.get_number_of_cells() {
                Ok(())
            } else {
                // index >= page num cells -> cursor pass last entry in whole Btree
                // TODO find a better mechanism to signal caller than bail with anyhow::Error
                bail!("iterated pass the last entry!")
            }
        } else if page_rc.borrow().is_interior() {
            // case current page is not a leaf page:
            // - extract child page number from the cell at current index of current page.
            // - move cursor to child page the cell at current index is pointing to.
            // - then, move cursor to leftmost cell of the child page.
            // Why leftmost? because in a B-Tree, the leftmost cell in a child page
            // is the next cell in ascending order after the parent cell.
            let child_page_num = self.get_child_page_num();
            self.move_to_child(child_page_num)?;
            self.move_to_left_most_leaf_entry()
        } else {
            bail!("page type invalid: not leaf nor interior!")
        }
    }

    fn get_child_page_num(&mut self) -> u32 {
        let page_rc = self.page_ref();
        let current_cell_ptr = page_rc
            .borrow_mut() // TODO really need mut?
            .get_cell_ptr(self.index_current_cell as usize);
        let child_page_num = u32::from_be_bytes(
            page_rc.borrow().data[current_cell_ptr..current_cell_ptr + 4]
                .try_into()
                .unwrap(),
        );
        child_page_num
    }

    /// Move cursor down to a new child page.
    /// child_page_no is the page number of the child page to move to.
    fn move_to_child(&mut self, child_page_no: u32) -> Result<()> {
        /*
        - update cursor state:
            - save current index and page in page stack and index stack
                pCur->aiIdx[pCur->iPage] = pCur->ix;
                pCur->apPage[pCur->iPage] = pCur->pPage;
            - reset current index to 0
            - increment page depth: pCur->iPage++;
        - calls getAndInitPage to fetch and initialize the child page child_page_no.
        getAndInitPage uses Pager to get page. Set BtCursor->pPage to that page.
        - check integrity of child page

        pCur->aiIdx u16 aiIdx[BTCURSOR_MAX_DEPTH-1]: array of u16 current cell indices at apPage[i]
        that cursor is accessing. apPage[iPage] is a pointer to the page at depth iPage
            -> BtCursor.cell_index_stack

        pCur->pPage: pointer to current page the cursor is at.
            -> BtCursor.page

        pCur->apPage MemPage *apPage[BTCURSOR_MAX_DEPTH-1]: an array of pointers to MemPage,
        stack of parents of current page.
            -> BtCursor.page_stack

        pCur->iPage i8: index/ depth of current page in apPage
            -> BtCursor.index_current_page

         */

        // update cursor state
        // Keeping 2 commented lines below as later we might want to use array for these stacks.
        // Currently not sure if these stacks index needs to map to exact Btree level.
        // For example, if moving from table root page to a child in level 4 directly,
        // do we need to save the page to index 3 of stack or just appending to stack is enought?
        // self.cell_index_stack[self.index_current_page as usize] = self.index_current_cell;
        // self.page_stack[self.index_current_page as usize] = self.page.clone();

        // currently just append to the stacks. see above for more details.
        self.cell_index_stack.push(self.index_current_cell);
        self.page_stack.push(self.page.clone());
        self.index_current_cell = 0;
        self.index_current_page += 1;

        // calling BufferPool to get the page and save the ref to BtCursor state
        let child_page_id = PageId::new(child_page_no);
        let mut db_ref = self.database.borrow_mut();
        let page = db_ref.buffer_pool.get_page(child_page_id);
        self.page = page;

        // TODO child page integrity check

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

    /// Move the cursor to the right-most leaf entry beneath the page
    /// to which it is pointing.
    ///
    /// The right-most entry is the one with the largest key -
    /// the last key in ascending order.
    fn move_to_right_most_leaf_entry(&mut self) -> Result<()> {
        // case 1: cursor is at interior page
        //  -> loop to move cursor to child page until reaching leaf page
        while self.page.borrow().is_interior() {
            // get page number of rightmost child page of current interior page.
            // Knowing this is an interior page, the option is not none, just unwrap.
            let right_child_page_no = self
                .page
                .borrow()
                .page_header
                .right_child_page_number
                .unwrap();
            info!(
                "Page {}, right child page num {right_child_page_no}",
                self.index_current_page
            );
            // Set index to one past last valid cell index in page to indicate that
            // cursor has visited all cells on the current page.
            // Also, this cell index in current page will be saved in move_to_child
            // cell_index_stack[index_current_page] = index_current_cell
            //                      parent              saving this cell index in parent
            // so that when cursor comes back to this page, it knows where it was last time.
            self.index_current_cell = self.page.borrow().get_number_of_cells();
            self.move_to_child(right_child_page_no)?;
        }

        // case 2: cursor is at leaf page (no page beneath)
        //  -> set cursor index to last cell index in current leaf page
        // cell index in SQlite in 0-indexed -> minus 1
        let last_cell_index = self.page.borrow().get_number_of_cells() - 1;
        self.index_current_cell = last_cell_index;

        Ok(())
    }

    /// Move cursor to left-most leaf entry one level beneath currency entry
    /// the cursor is pointing to.
    ///
    /// The left-most leaf is the one with the smallest key -
    /// the first in ascending order.
    ///
    /// Equivalent to sqlite `static int moveToLeftmost(BtCursor *pCur)`
    fn move_to_left_most_leaf_entry(&mut self) -> Result<()> {
        let mut action_result = Ok(());

        loop {
            if self.page.borrow().is_leaf() || action_result.is_err() {
                break;
            }
            assert!(self.index_current_cell < self.page.borrow().get_number_of_cells());
            let child_page_no = self.get_child_page_num();
            action_result = self.move_to_child(child_page_no);
        }

        action_result
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

    use crate::test_utils::setup;

    use super::*;

    const TABLE_SUPERHEROES_ROOT_PAGE: u32 = 2;
    const TABLE_APPLES_ROOT_PAGE: u32 = 2;

    fn db_ref_sample() -> Rc<RefCell<Database>> {
        // sample.db has 2 tables: apples and oranges. Each one in 1 leaf page.
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();
        Rc::new(RefCell::new(db))
    }

    fn db_ref_superheroes() -> Rc<RefCell<Database>> {
        // superheroes.db has table spanning > 1 page
        let db_path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/superheroes.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();
        Rc::new(RefCell::new(db))
    }

    #[test]
    fn test_scan() {
        let mut cursor = BtCursor::new(db_ref_superheroes().clone(), 2);

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
        let cursor = BtCursor::new(db_ref_superheroes().clone(), 0);
        assert_eq!(cursor.root_page_number, 0);

        // should work when cursor moved away from root page
        let mut cursor = BtCursor::new(db_ref_superheroes().clone(), 2);
        assert_eq!(cursor.root_page_number, 2);
        cursor.move_to_last().unwrap();
        assert_eq!(cursor.root_page_number, 2);
    }

    #[test]
    fn test_move_to_child_ok() {
        let mut cursor = BtCursor::new(db_ref_superheroes().clone(), TABLE_SUPERHEROES_ROOT_PAGE);

        cursor.move_to_child(3).unwrap();
        assert_eq!(cursor.root_page_number, TABLE_SUPERHEROES_ROOT_PAGE);
        assert!(cursor.page.borrow().is_leaf());
        assert_eq!(cursor.page.borrow().page_id.page_number, 3);
        println!("{:?}", cursor.page.borrow());

        cursor.move_to_child(5).unwrap();
        assert_eq!(cursor.root_page_number, TABLE_SUPERHEROES_ROOT_PAGE);
        assert!(cursor.page.borrow().is_leaf());
        assert_eq!(cursor.page.borrow().page_id.page_number, 5);
        println!("{:?}", cursor.page.borrow());
    }

    fn assert_cell_byte_offset(cursor: &BtCursor, expected_offset: usize) {
        let mut page = cursor.page.borrow_mut();
        let cell_byte_offset = page.get_cell_ptr(cursor.index_current_cell as usize);
        assert_eq!(cell_byte_offset, expected_offset);
        println!("{cursor:#?}")
    }

    fn assert_cursor_points_to_leaf_page(cursor: &BtCursor) {
        assert!(cursor.page.borrow().is_leaf());
    }

    #[test]
    fn test_move_to_next_table_single_page() {
        let table_apples_root_page = 2;
        let mut cursor = BtCursor::new(db_ref_sample().clone(), table_apples_root_page);

        /*
        To parse cell content from cursor
        1. Get Page that cursor is pointing to.
        2. Find pointer to the cell from index_current_cell (0, 1, etc.) and Page
        using Page.get_cell_ptr fn.
        3. Parse cell content from cell byte offset and page bytes.
         */

        // table apples has 4 cells [4067, 4054, 4029, 4001]
        // first cell
        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        assert_eq!(cursor.index_current_cell, 0);
        assert_cell_byte_offset(&cursor, 4067);

        // second cell
        cursor.move_to_next().unwrap();
        assert_cursor_points_to_leaf_page(&cursor);
        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        assert_eq!(cursor.index_current_cell, 1);
        assert_cell_byte_offset(&cursor, 4054);

        // third cell
        cursor.move_to_next().unwrap();
        assert_cursor_points_to_leaf_page(&cursor);
        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        assert_eq!(cursor.index_current_cell, 2);
        assert_cell_byte_offset(&cursor, 4029);

        // forth cell
        cursor.move_to_next().unwrap();
        assert_cursor_points_to_leaf_page(&cursor);
        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        assert_eq!(cursor.index_current_cell, 3);
        assert_cell_byte_offset(&cursor, 4001);

        // moving past the last entry should be an error
        assert!(cursor.move_to_next().is_err());
    }

    #[test]
    fn test_move_to_next_table_2_pages() {}

    #[test]
    fn test_move_to_previous() {}

    #[test]
    fn test_move_to_last() {}

    #[test]
    fn test_move_to_first() {}

    #[test]
    fn test_move_to_right_most_table_single_page() {
        let mut cursor = BtCursor::new(db_ref_sample().clone(), TABLE_APPLES_ROOT_PAGE);
        cursor.move_to_right_most_leaf_entry().unwrap();

        // Check if cursor is at the right-most leaf entry
        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        // table apples has 4 entries = 4 cells -> last cell index is 3 (0-indexed)
        assert_eq!(cursor.index_current_cell, 3);
    }

    #[test]
    fn test_move_to_right_most_table_many_pages() {
        setup();
        let mut cursor = BtCursor::new(db_ref_superheroes().clone(), TABLE_SUPERHEROES_ROOT_PAGE);

        assert_eq!(cursor.page.borrow().page_id.page_number, 2);
        assert!(cursor.page.borrow().is_interior());
        assert_eq!(cursor.index_current_cell, 0);

        cursor.move_to_right_most_leaf_entry().unwrap();
        // rightmost leaf page number of table superherous is 232
        assert_eq!(cursor.page.borrow().page_id.page_number, 232);
        assert!(cursor.page.borrow().is_leaf());
        // rightmost leaf page has xxx cells -> index  (0-indexed)
        assert_eq!(cursor.index_current_cell, 48);
    }
}

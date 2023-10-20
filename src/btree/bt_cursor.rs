use std::cell::RefCell;
use std::rc::Rc;

use crate::btree::btree::BTree;
use crate::model::page::Page;

pub struct BtCursor {
    // Rc for multiple references to same page
    // RefCell allows mutable borrowing because we would want to modify the BTree
    // and Page when we have a reference of it.
    btree: Rc<RefCell<BTree>>,
    page: Option<Rc<RefCell<Page>>>,
}

impl BtCursor {
    pub fn new() -> Self {
        todo!()
    }

    pub fn move_to_next(&mut self) -> Option<Rc<RefCell<Page>>> {
        // Move the cursor to the next cell
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
}
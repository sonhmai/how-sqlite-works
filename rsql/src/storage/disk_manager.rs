use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use anyhow::Result;

use crate::model::page::Page;
use crate::model::page_id::PageId;

/// Shared ownership and we want to mutate DiskManager (e.g. for writing)
pub type SharedDiskManager = Rc<RefCell<dyn DiskManager>>;

pub trait DiskManager: Debug {
    fn read_page(&self, page_id: PageId) -> Result<Page>;

    fn write_page(&mut self, page_id: PageId, page: &Page) -> Result<()>;
}

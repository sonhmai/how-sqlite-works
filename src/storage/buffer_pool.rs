use std::collections::HashMap;
use crate::model::database::Database;
use crate::model::page::Page;
use crate::model::page_id::PageId;

/// BufferPool manages reading and writing of pages into mem from disk.
/// Access methods (scan, write, etc.) call it to get, write pages.
///
/// BufferPool also is responsible for locking.
/// Then transaction fetches a page, it checks whether transaction has lock.
pub struct BufferPool {
    // current not supporting concurrency
    page_map: HashMap<PageId, Page>,
    database: &'static Database,
}

impl BufferPool {
    pub fn get_page(&self, page_id: PageId) -> Page {
        self.database.read_page(page_id)
    }
}
use std::collections::HashMap;
use crate::model::page::Page;
use crate::model::page_id::PageId;

/// BufferPool manages reading and writing of pages into mem from disk.
/// Access methods (scan, write, etc.) call it to get, write pages.
///
/// BufferPool also is responsible for locking.
/// Then transaction fetches a page, it checks whether transaction has lock.
pub struct BufferPool {
    // current not supporting concurrency
    page_map: HashMap<PageId, Page>
}

impl BufferPool {
    pub fn get_page(page_id: PageId) -> Page {
        todo!()
    }
}
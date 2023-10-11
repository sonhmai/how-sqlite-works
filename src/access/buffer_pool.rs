use std::collections::HashMap;

use crate::model::page::Page;
use crate::model::page_id::PageId;
use crate::storage::disk_manager::DiskManager;

/// BufferPool manages buffering (caching) of pages into mem from disk.
///
/// Access methods (scan, write, etc.) call it to read, write pages,
/// not directly to DiskManager.
///
/// BufferPool also is has reference to LockManager for concurrency control.
/// Then transaction fetches a page, it checks whether transaction has lock.
#[derive(Debug)]
pub struct BufferPool {
    // current not supporting concurrency - HashMap
    page_map: HashMap<PageId, Page>,
    disk_manager: DiskManager,
}

impl BufferPool {
    // DiskManager lifetime must be at least as long as BufferPool
    pub fn new(disk_manager: DiskManager) -> Self {
        BufferPool {
            page_map: HashMap::new(),
            disk_manager,
        }
    }
    pub fn get_page(&self, page_id: PageId) -> Page {
        self.disk_manager.read_page(page_id).unwrap()
    }
}
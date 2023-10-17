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
    // current not supporting concurrency
    page_table: HashMap<PageId, Page>,  // page_table keeping track of page in-mem caching
    disk_manager: DiskManager,
}

impl BufferPool {
    /// Creates a BufferPool that caches up to capacity pages.
    /// DiskManager lifetime must be at least as long as BufferPool
    pub fn new(capacity: u32, disk_manager: DiskManager) -> Self {
        BufferPool {
            page_table: HashMap::new(),
            disk_manager,
        }
    }

    /// Reads a page.
    /// Retrieved page is looked up and returned if available in memory cache.
    /// If not in cache, it should be read from the DiskManager, saved in buffer,
    /// then return. If there are insufficient buffer space, a page in the buffer
    /// should be evicted based on the policy and new page added.
    pub fn get_page(&self, page_id: PageId) -> Page {
        self.disk_manager.read_page(page_id).unwrap()
    }

    /// Flushes a page to disk.
    pub fn flush_page(&self, page_id: PageId) {
        todo!()
    }

    /// Checks if a page is in the buffer.
    fn have_page(&self, page_id: PageId) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_buffer_pool_evict_page_when_over_capacity() {
        // TODO
    }
}
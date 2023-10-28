use std::cell::RefCell;
use std::num::NonZeroUsize;
use std::rc::Rc;

use lru::LruCache;

use crate::model::page::Page;
use crate::model::page_id::PageId;
use crate::storage::disk_manager::{DiskManager, SharedDiskManager};
use crate::wal::wal::Wal;

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
    page_table: LruCache<PageId, Rc<RefCell<Page>>>,  // page_table keeping track of page in-mem caching
    disk_manager: SharedDiskManager,
    // When a transaction is committed, all dirty pages (modified in mem not written to disk)
    // are gathered and written to WAL.
    wal: Wal,
}

impl BufferPool {
    /// Creates a BufferPool that caches up to capacity pages.
    /// DiskManager lifetime must be at least as long as BufferPool
    pub fn new(capacity: usize, disk_manager: SharedDiskManager) -> Self {
        let capacity = NonZeroUsize::new(capacity as usize).expect("Capacity must be non-zero");
        BufferPool {
            page_table: LruCache::new(capacity),
            disk_manager: disk_manager.clone(),
            wal: Wal::new(disk_manager.clone()).unwrap(),
        }
    }

    /// Reads a page.
    /// Retrieved page is looked up and returned if available in memory cache.
    /// If not in cache, it should be read from the DiskManager, saved in buffer,
    /// then return. If there are insufficient buffer space, a page in the buffer
    /// should be evicted based on the policy and new page added.
    pub fn get_page(&mut self, page_id: PageId) -> Rc<RefCell<Page>> {
        if !self.page_table.contains(&page_id) {
            let page = self.disk_manager
                .borrow_mut()
                .read_page(page_id)
                .expect("Failed to read page from disk");
            let mut_page_ref = Rc::new(RefCell::new(page));
            self.page_table.put(page_id, mut_page_ref.clone());
        }
        self.page_table
            .get(&page_id)
            .expect("Failed to get page from buffer pool")
            .clone()
    }

    /// Flushes a page to disk.
    pub fn flush_page(&mut self, page_id: PageId) {
        if let Some(page) = self.page_table.pop(&page_id) {
            /*
            page is of type Rc<RefCell<Page>>.
            To get a reference to the Page inside, borrow() on the RefCell -> a Ref<Page>.
            To get a Page from a Ref<Page>, use * to dereference it.
            Finally, to get a reference to a Page from a Page, use &.
            TODO - change disk_manager method, maybe it should take Rc<RefCell<Page>>?
             */
            self.disk_manager
                .borrow_mut()
                .write_page(page_id, &*page.borrow())
                .unwrap();
        }
    }

    /// Checks if a page is in the buffer.
    fn have_page(&self, page_id: PageId) -> bool {
        self.page_table.contains(&page_id)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::storage::disk_manager::DiskManager;

    use super::*;

    fn ref_disk_manager() -> SharedDiskManager {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let disk_manager = DiskManager::new(
            db_path.to_str().unwrap(), 4096).unwrap();
        let dm_ref = Rc::new(RefCell::new(disk_manager));
        dm_ref.clone()
    }

    #[test]
    fn test_buffer_pool_evict_page_when_over_capacity() {
        let mut buffer_pool = BufferPool::new(2, ref_disk_manager().clone());

        buffer_pool.get_page(PageId { page_number: 4 });
        buffer_pool.get_page(PageId { page_number: 2 });
        buffer_pool.get_page(PageId { page_number: 3 });
        
        // should evict first page added because of 2 capacity
        assert_eq!(buffer_pool.have_page(PageId { page_number: 4 }), false);
        assert_eq!(buffer_pool.have_page(PageId { page_number: 2 }), true);
        assert_eq!(buffer_pool.have_page(PageId { page_number: 3 }), true);
    }
}
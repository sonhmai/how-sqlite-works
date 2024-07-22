use crate::model::page::Page;
use crate::model::page_id::PageId;
use crate::storage::disk_manager::DiskManager;

/// No Op DiskManager that does nothing.
#[derive(Debug)]
pub struct NoOpDiskManager {}

impl DiskManager for NoOpDiskManager {
    fn read_page(&self, _page_id: PageId) -> anyhow::Result<Page> {
        Ok(Page::dummy())
    }

    fn write_page(&mut self, _page_id: PageId, _page: &Page) -> anyhow::Result<()> {
        Ok(())
    }
}

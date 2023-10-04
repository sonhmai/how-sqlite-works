use crate::model::page_id::PageId;

/// A page in the SQLite database.
/// https://www.sqlite.org/fileformat.html#pages
///
/// Pages are numbered starting with 1 (there is no page 0).
/// The maximum page number is 4294967294 (2^32 - 2).
/// The minimum size SQLite database is a single 512-byte page.
pub struct Page {
    page_id: PageId
}

impl Page {

    fn new(page_id: PageId) -> Self {
        Page {
            page_id
        }
    }

    // Return the id of this page.
    // The id is a unique identifier for a page that can be used to
    // - look up the page on disk
    // - or determine if the page exists in the buffer pool.
    fn get_id(&self) -> PageId {
        self.page_id
    }
}

/// A page number starting with 1 uniquely identifies a page
/// because a sqlite database is a single file.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PageId {
    pub page_number: u32
}

impl PageId {

}
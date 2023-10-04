use anyhow::{bail, Result};

// https://github.com/richardmarbach/codecrafters-sqlite-rust/blob/master/src/page.rs#L161
// https://github.com/bert2/build-your-own-sqlite-rust/blob/master/src/format/page_header.rs
#[derive(Debug)]
pub struct PageHeader{
    pub page_type: PageType,
    pub first_free_block_start: u32,
    pub number_of_cells: u32,
    pub content_start_offset: usize,
    pub fragment_free_bytes: usize,
    pub right_child_page_number: Option<u32>
}

impl PageHeader {
    pub fn parse(stream: &[u8]) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub enum PageType {
    InteriorIndex = 2,
    InteriorTable = 5,
    LeafIndex = 10,
    LeafTable = 13,
}
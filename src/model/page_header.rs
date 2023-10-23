use std::u8;

use anyhow::{bail, Result};

#[derive(Debug)]
pub struct PageHeader {
    pub page_type: PageType,
    pub first_free_block_start: u16,
    pub number_of_cells: u16,
    pub content_start_offset: u16,
    pub fragmented_free_bytes: u8,
    // right_child_page_number available only when page is an interior table page or index page
    pub right_child_page_number: Option<u32>,
}

impl PageHeader {
    pub fn parse(stream: &[u8]) -> Result<Self> {
        let page_type = match stream[0] {
            2 => PageType::InteriorIndex,
            5 => PageType::InteriorTable,
            10 => PageType::LeafIndex,
            13 => PageType::LeafTable,
            x => bail!("Invalid page type encountered: {}", x),
        };
        let first_free_block_start = u16::from_be_bytes(stream[1..3].try_into()?);
        let number_of_cells = u16::from_be_bytes(stream[3..5].try_into()?);
        let content_start_offset = u16::from_be_bytes(stream[5..7].try_into()?);
        let fragmented_free_bytes = stream[7];
        let right_child_page_number = match page_type {
            PageType::InteriorTable | PageType::InteriorIndex => {
                Some(u32::from_be_bytes(stream[8..12].try_into()?))
            }
            _ => None,
        };

        Ok(Self {
            page_type,
            first_free_block_start,
            number_of_cells,
            content_start_offset,
            fragmented_free_bytes,
            right_child_page_number,
        })
    }

    // not sure why this fn is qualified with  const
    // https://doc.rust-lang.org/std/keyword.const.html#compile-time-evaluable-functions
    pub const fn is_leaf(&self) -> bool {
        matches!(self.page_type, PageType::LeafIndex | PageType::LeafTable)
    }

    pub const fn is_table_leaf(&self) -> bool {
        matches!(self.page_type, PageType::LeafTable)
    }

    pub const fn is_table_interior(&self) -> bool {
        matches!(self.page_type, PageType::InteriorTable)
    }

    pub const fn is_index_leaf(&self) -> bool {
        matches!(self.page_type, PageType::LeafIndex)
    }

    pub const fn is_index_interior(&self) -> bool {
        matches!(self.page_type, PageType::InteriorIndex)
    }

    pub const fn size(&self) -> usize {
        if self.is_leaf() {
            8
        } else {
            12
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PageType {
    InteriorIndex = 2,
    InteriorTable = 5,
    LeafIndex = 10,
    LeafTable = 13,
}
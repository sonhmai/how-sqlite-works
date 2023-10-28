use anyhow::Result;

use crate::varint::decode_varint;

/// `CellTableInterior` represents a Table B-Tree Interior Cell (header 0x05).
/// - 4-byte big-endian page number which is the left child pointer,
/// - rowid - a varint which is the integer key.
/// Interior pages of table b-trees have no payload and so there is never any payload to spill.
/// https://www.sqlite.org/fileformat.html#b_tree_pages
#[derive(Debug)]
pub struct CellTableInterior {
    pub left_child_pointer: u32,
    pub rowid: i64,
}

impl CellTableInterior {
    /// Parses a byte stream into a `CellTableInterior`.
    pub fn parse(stream: &[u8]) -> Result<Self> {
        let left_child_pointer = u32::from_be_bytes(stream[0..4].try_into()?);
        let (rowid, _) = decode_varint(&stream[4..]);

        Ok(Self {
            left_child_pointer,
            rowid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell_table_interior() {
        let cell_bytes: Vec<u8> = vec![0, 0, 0, 1, 2]; // Add your test data here
        let cell = CellTableInterior::parse(cell_bytes.as_slice()).unwrap();
        assert_eq!(cell.left_child_pointer, 1);
        assert_eq!(cell.rowid, 2);
    }
}

/*
https://www.sqlite.org/fileformat2.html#walformat
 */
use anyhow::Result;

/// 24-byte frame-header.
#[derive(Debug, Clone)]
pub struct WalFrameHeader {
    pub page_number: u32,
    // For commit records, the size of the database file in pages after the commit.
    // For all other records, zero.
    pub db_size_after_commit: u32,
    pub salt_1: u32, // Salt-1 copied from the WAL header: random integer incremented with each checkpoint
    pub salt_2: u32, // Salt-2 from the WAL header: a different random number for each checkpoint
    pub checksum_1: u32, // Cumulative checksum up through and including this page
    pub checksum_2: u32, // Second half of the cumulative checksum.
}

impl WalFrameHeader {

    pub const SIZE: usize = 24;

    pub fn from_bytes(bytes: &[u8; 24]) -> Result<Self> {
        Ok(WalFrameHeader {
            page_number: u32::from_be_bytes(bytes[0..4].try_into()?),
            db_size_after_commit: u32::from_be_bytes(bytes[4..8].try_into()?),
            salt_1: u32::from_be_bytes(bytes[8..12].try_into()?),
            salt_2: u32::from_be_bytes(bytes[12..16].try_into()?),
            checksum_1: u32::from_be_bytes(bytes[16..20].try_into()?),
            checksum_2: u32::from_be_bytes(bytes[20..24].try_into()?),
        })
    }
}

/// A WAL has zero or more WalFrame.
/// 24-byte frame-header followed by a page-size bytes of page data.
///
/// WalFrame size = 24B (header) + page_size in bytes.
/// For example, a wal_frame has size of 4120B in case db page size is 4096B.
///
/// A frame is considered valid if and only if the following conditions are true:
///   1. The salt-1 and salt-2 values in the frame-header match salt values in the wal-header.
///   2. The checksum values in the final 8 bytes of the frame-header exactly match
///   the checksum computed consecutively on the first 24 bytes of the WAL header and
///   the first 8 bytes and the content of all frames up to and including the current frame.
#[derive(Debug)]
pub struct WalFrame {
    pub header: WalFrameHeader,
    pub data: Vec<u8>,
}

impl WalFrame {
    pub fn from_bytes(bytes: &[u8], page_size: usize) -> Result<Self> {
        let header = WalFrameHeader::from_bytes(bytes[0..24].try_into()?)?;

        Ok(WalFrame {
            header,
            // TODO copying the bytes allocates extra memory on heap.
            //  Should a Page is parsed here?
            data: bytes[WalFrameHeader::SIZE..WalFrameHeader::SIZE + page_size].to_vec()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::file_bytes_vec;
    use crate::wal::wal_frame::WalFrame;

    #[test]
    fn test_parse_single_wal_frame() {
        let wal_bytes = file_bytes_vec("tests/resources/apples_wal.db-wal");
        let page_size = 4096;
        let wal_frame = WalFrame::from_bytes(
            &wal_bytes.as_slice()[32..], // skip first 32 bytes of WalHeader
            page_size
        ).unwrap();

        println!("{wal_frame:?}");

        assert_eq!(wal_frame.header.page_number, 2);
        assert_eq!(wal_frame.header.db_size_after_commit, 2);
    }
}

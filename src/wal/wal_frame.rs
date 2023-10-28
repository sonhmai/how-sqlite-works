/*
https://www.sqlite.org/fileformat2.html#walformat
 */

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

/// A WAL has zero or more WalFrame.
/// 24-byte frame-header followed by a page-size bytes of page data.
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

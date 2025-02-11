/*
[WAL File Format](https://www.sqlite.org/fileformat2.html#walformat)
- a header + zero or more frames.
- each frame = revised content of a single page from db file.
- Transactions commit when a frame is written that contains a commit marker.
A single WAL can and usually does record multiple transactions.
- content of the WAL is transferred back into the database file in an operation called a "checkpoint".
- a WAL file always grows from beginning to end -> sequential disk IO.
- each frame has checksum for data integrity and counter for know whether it's checkpointed.
 */

use anyhow::Result;

use crate::storage::disk_manager::SharedDiskManager;
use crate::wal::wal_frame::{WalFrame, WalFrameHeader};
use crate::wal::wal_header::WalHeader;

/// Represent a open write-ahead log file of a database.
/// One open database should have only one Wal object.
#[derive(Debug)]
pub struct Wal {
    pub header: WalHeader,
    pub frames: Vec<WalFrame>,
    pub disk_manager: SharedDiskManager,
}

impl Wal {
    pub fn new(disk_manager: SharedDiskManager) -> Result<Self> {
        // TODO open existing or  create new WAL file
        // TODO read the WAL header

        // temp: create a dummy header for now
        let header = WalHeader::new(0x377f0682, 3007000, 4096, 1, 12345, 67890, 11111, 22222);

        Ok(Wal {
            disk_manager,
            frames: vec![],
            header,
        })
    }

    /// Create Wal from bytes.
    ///
    /// bytes: byte slice of wal file, not containing other things.
    pub fn from_bytes(bytes: &[u8], disk_manager: SharedDiskManager) -> Result<Self> {
        let header = WalHeader::from_bytes(bytes[0..WalHeader::SIZE].try_into()?);
        let wal_n_bytes = bytes.len();
        println!("{wal_n_bytes}");
        let frame_size = header.page_size as usize + WalFrameHeader::SIZE;
        let n_frames = (wal_n_bytes - WalHeader::SIZE) / frame_size;

        let mut frames: Vec<WalFrame> = vec![];
        let mut bytes_cursor = WalHeader::SIZE;

        for _ in 0..n_frames {
            let wal_frame = WalFrame::from_bytes(
                &bytes[bytes_cursor..bytes_cursor + frame_size],
                header.page_size as usize,
            )?;
            frames.push(wal_frame);
            bytes_cursor += frame_size;
        }

        Ok(Wal {
            disk_manager,
            frames, // TODO parse wal frames from bytes
            header,
        })
    }

    /// Write a set of frames to the log. Called when a transaction is committed.
    pub fn write_frames(&mut self) -> Result<()> {
        Ok(())
    }

    /// Undo frames written but not committed yet to the wal log.
    /// Used to rollback a transaction.
    pub fn undo(&mut self) -> Result<()> {
        Ok(())
    }

    /// Checkpoint wal log = copy pages from log to db file.
    /// Transfer the changes recording in wal log to main db file.
    pub fn checkpoint(&mut self) -> Result<()> {
        Ok(())
    }

    /// Close connection to a log file.
    ///
    /// Why need an explicit close?
    /// - Data Integrity: writing remaining data in WAL back to database before
    /// ending current session.
    /// - Resource Management: releasing resources like file handle, etc.
    pub fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::storage::no_op::NoOpDiskManager;
    use crate::test_utils::file_bytes_vec;

    use super::*;

    #[test]
    fn test_parse_simple_wal_from_file() {
        let wal_bytes = file_bytes_vec("tests/resources/apples_wal.db-wal");
        let dummy_dm = Rc::new(RefCell::new(NoOpDiskManager {}));
        let wal = Wal::from_bytes(wal_bytes.as_slice(), dummy_dm).unwrap();

        println!("{wal:?}");

        assert_eq!(wal.header.page_size, 4096);
        assert_eq!(wal.header.checkpoint_seq, 0);
        assert_eq!(wal.header.file_format, 3007000);

        assert_eq!(wal.frames.len(), 1);

        /*
        A frame is considered valid if and only if the following conditions are true:
          1. The salt-1 and salt-2 values in the frame-header match salt values in the wal-header.
          2. The checksum values in the final 8 bytes of the frame-header exactly match
          the checksum computed consecutively on the first 24 bytes of the WAL header and
          the first 8 bytes and the content of all frames up to and including the current frame.
         */
        assert_eq!(wal.frames[0].header.salt_1, wal.header.salt_1);
        assert_eq!(wal.frames[0].header.salt_2, wal.header.salt_2);
    }
}

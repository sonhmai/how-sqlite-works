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

use crate::wal::wal_frame::WalFrame;

#[derive(Debug, Clone)]
pub struct WalHeader {
    // whole header 32 bytes in size, all values big-endian
    pub magic_number: u32, // 0x377f0682 or 0x377f0683
    pub file_format: u32, // currently 3007000 as in sqlite doc
    pub page_size: u32, // db page size
    pub checkpoint_seq: u32, // checkpoint sequence number
    pub salt_1: u32, // random integer incremented with each checkpoint
    pub salt_2: u32, // a difference random number for each checkpoint
    pub checksum_1: u32,
    pub checksum_2: u32,
}

/// Represent a open write-ahead log file of a database.
/// One open database should have only one Wal object.
#[derive(Debug)]
pub struct Wal {
    pub header: WalHeader,
    pub frames: Vec<WalFrame>,
    pub disk_manager: SharedDiskManager,
}

impl Wal {

    pub fn new() -> Result<Self> {
        todo!()
        // open existing or  create new WAL file
        // read the WAL header
        // return Wal object
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
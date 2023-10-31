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
use std::fs::File;
use std::io::{Read, Write};

use anyhow::Result;

use crate::storage::disk_manager::SharedDiskManager;
use crate::wal::wal_frame::WalFrame;

#[derive(Debug, Clone)]
pub struct WalHeader {
    // whole header = 8x u32 = 32 bytes in size, all values big-endian
    pub magic_number: u32,
    // 0x377f0682 or 0x377f0683
    pub file_format: u32,
    // currently 3007000 as in sqlite doc
    pub page_size: u32,
    // db page size
    pub checkpoint_seq: u32,
    // checkpoint sequence number
    pub salt_1: u32,
    // random integer incremented with each checkpoint
    pub salt_2: u32,
    // a difference random number for each checkpoint
    pub checksum_1: u32,
    pub checksum_2: u32,
}

impl WalHeader {
    pub fn new(
        magic_number: u32,
        file_format: u32,
        page_size: u32,
        checkpoint_seq: u32,
        salt_1: u32,
        salt_2: u32,
        checksum_1: u32,
        checksum_2: u32,
    ) -> Self {
        WalHeader {
            magic_number,
            file_format,
            page_size,
            checkpoint_seq,
            salt_1,
            salt_2,
            checksum_1,
            checksum_2,
        }
    }
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        WalHeader {
            magic_number: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            file_format: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            page_size: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
            checkpoint_seq: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            salt_1: u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
            salt_2: u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
            checksum_1: u32::from_be_bytes(bytes[24..28].try_into().unwrap()),
            checksum_2: u32::from_be_bytes(bytes[28..32].try_into().unwrap()),
        }
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes[0..4].copy_from_slice(&self.magic_number.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.file_format.to_be_bytes());
        bytes[8..12].copy_from_slice(&self.page_size.to_be_bytes());
        bytes[12..16].copy_from_slice(&self.checkpoint_seq.to_be_bytes());
        bytes[16..20].copy_from_slice(&self.salt_1.to_be_bytes());
        bytes[20..24].copy_from_slice(&self.salt_2.to_be_bytes());
        bytes[24..28].copy_from_slice(&self.checksum_1.to_be_bytes());
        bytes[28..32].copy_from_slice(&self.checksum_2.to_be_bytes());
        bytes
    }

    pub fn write_to_file(&self, file: &mut File) -> Result<()> {
        file.write_all(&self.to_bytes())?;
        Ok(())
    }

    pub fn read_from_file(file: &mut File) -> Result<Self> {
        // read_exact needs a mut ref File because it needs to modify its internal state.
        // It changes the current position in the file/ stream, this pos stored as state.
        let mut bytes = [0u8; 32];
        file.read_exact(&mut bytes)?;
        Ok(Self::from_bytes(&bytes))
    }
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

    pub fn from_bytes(bytes: &[u8], disk_manager: SharedDiskManager) -> Result<Self> {
        let header = WalHeader::from_bytes(bytes[0..32].try_into()?);

        Ok(Wal {
            disk_manager,
            frames: vec![], // TODO parse wal frames from bytes
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
    use std::io::Seek;
    use std::rc::Rc;

    use tempfile::tempfile;
    use crate::storage::no_op::NoOpDiskManager;

    use crate::test_utils::file_bytes_vec;

    use super::*;

    #[test]
    fn test_wal_header() {
        let header_mem = WalHeader::new(0x377f0682, 3007000, 4096, 1, 12345, 67890, 11111, 22222);
        let mut temp_file = tempfile().unwrap();
        header_mem.write_to_file(&mut temp_file).unwrap();
        temp_file.seek(std::io::SeekFrom::Start(0)).unwrap(); // Reset the file cursor to the start
        let header_deser_file = WalHeader::read_from_file(&mut temp_file).unwrap();
        assert_eq!(header_mem.magic_number, header_deser_file.magic_number);
        assert_eq!(header_mem.file_format, header_deser_file.file_format);
        assert_eq!(header_mem.page_size, header_deser_file.page_size);
        assert_eq!(header_mem.checkpoint_seq, header_deser_file.checkpoint_seq);
        assert_eq!(header_mem.salt_1, header_deser_file.salt_1);
        assert_eq!(header_mem.salt_2, header_deser_file.salt_2);
        assert_eq!(header_mem.checksum_1, header_deser_file.checksum_1);
        assert_eq!(header_mem.checksum_2, header_deser_file.checksum_2);
    }

    #[test]
    fn test_parse_simple_wal_from_file() {
        let wal_bytes = file_bytes_vec("tests/resources/apples_wal.db-wal");
        let dummy_dm = Rc::new(RefCell::new(NoOpDiskManager{}));
        let wal = Wal::from_bytes(wal_bytes.as_slice(), dummy_dm).unwrap();

        println!("{wal:?}");

        assert_eq!(wal.header.page_size, 4096);
        assert_eq!(wal.header.checkpoint_seq, 0);
        assert_eq!(wal.header.file_format, 3007000);
    }
}

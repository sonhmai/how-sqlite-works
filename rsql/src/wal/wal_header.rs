use std::fs::File;
use std::io::{Read, Write};

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
    pub const SIZE: usize = 32;

    #[allow(clippy::too_many_arguments)]
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

    pub fn write_to_file(&self, file: &mut File) -> anyhow::Result<()> {
        file.write_all(&self.to_bytes())?;
        Ok(())
    }

    pub fn read_from_file(file: &mut File) -> anyhow::Result<Self> {
        // read_exact needs a mut ref File because it needs to modify its internal state.
        // It changes the current position in the file/ stream, this pos stored as state.
        let mut bytes = [0u8; 32];
        file.read_exact(&mut bytes)?;
        Ok(Self::from_bytes(&bytes))
    }
}

#[cfg(test)]
mod tests {
    use std::io::Seek;

    use tempfile::tempfile;

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
}

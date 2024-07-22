use std::mem;

use anyhow::{bail, Result};

const WAL_CKSUM_OFFSET: usize = 24;

/// Calculate the checksum for a database page.
/// The calculation is done in-place on the given page buffer.
pub fn wal_checksum_bytes(cksum: &mut u32, data: &mut [u8]) {
    let mut checksum = *cksum;
    for &byte in data.iter() {
        checksum = checksum.wrapping_add(byte as u32);
        checksum = (checksum << 1) | (checksum >> 31);
    }
    *cksum = checksum;
}

/// Verify the checksum of a database page.
pub fn wal_verify_page(data: &[u8]) -> Result<()> {
    let mut cksum = 0u32;

    // Calculate the checksum of the page content
    let page_data = &data[0..WAL_CKSUM_OFFSET];
    wal_checksum_bytes(&mut cksum, &mut page_data.to_owned());

    // Read the saved checksum value from the page header
    let saved_cksum_bytes = &data[WAL_CKSUM_OFFSET..WAL_CKSUM_OFFSET + mem::size_of::<u32>()];
    let saved_cksum = u32::from_le_bytes(saved_cksum_bytes.try_into().unwrap());

    // Compare the calculated checksum with the saved checksum
    if cksum != saved_cksum {
        bail!("Checksum verification failed")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_verify_checksum() {}
}

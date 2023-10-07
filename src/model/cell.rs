use anyhow::Result;

use crate::model::data_record::DataRecord;
use crate::varint::decode_varint;

/*
LeafTableCell (header 0x0d) has format (in order of appearance)
    -A varint which is the total number of bytes of payload, including any overflow
    -A varint which is the integer key, a.k.a. "rowid"
    -The initial portion of the payload that does not spill to overflow pages.
    -A 4-byte big-endian integer page number for the first page of
    the overflow page list - omitted if all payload fits on the b-tree page.
 */
#[derive(Debug)]
pub struct LeafTableCell {
    pub rowid: i64,
    pub payload: DataRecord,
}

impl LeafTableCell {
    // not handling payload overflow yet (payload bigger than 1 page)
    pub fn parse(stream: &[u8]) -> Result<Self> {
        let mut offset = 0;
        // payload size first
        let (payload_size, bytes_read) = decode_varint(stream);
        let payload_size: usize = payload_size.try_into()?;
        offset += bytes_read;
        // then rowid
        let (rowid, bytes_read) = decode_varint(&stream[offset..]);
        offset += bytes_read;
        // then payload with Record format, not handling overflow
        let payload = DataRecord::parse_from(
            rowid.unsigned_abs(),
            &stream[offset..offset + payload_size]
        );

        Ok(Self {
            rowid,
            payload,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_leaf_table_cell() {
        
    }
}
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
        let payload =
            DataRecord::parse_from(rowid.unsigned_abs(), &stream[offset..offset + payload_size]);

        Ok(Self { rowid, payload })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::cell_table_leaf::LeafTableCell;
    use crate::model::column_value::ColumnValue;
    use log::info;

    #[test]
    fn test_parse_leaf_table_cell_apples_sample_db() {
        // leaf table cells from sample db apples table.

        // rowid 2
        // values: [Null, Text("Fuji"), Text("Red")]
        let cell_bytes: Vec<u8> = vec![11, 2, 4, 0, 21, 19, 70, 117, 106, 105, 82, 101, 100];
        let cell = LeafTableCell::parse(cell_bytes.as_slice()).unwrap();
        // expected LeafTableCell {
        //  rowid: 2,
        //  payload: DataRecord { values: [Null, Text("Fuji"), Text("Red")], rowid: Some(2) } }
        assert_eq!(cell.rowid, 2);
        assert_eq!(cell.payload.values.len(), 3);
        assert_eq!(cell.payload.values[0], ColumnValue::Null);
        assert_eq!(cell.payload.values[1], ColumnValue::Text("Fuji".to_owned()));
        assert_eq!(cell.payload.values[2], ColumnValue::Text("Red".to_owned()));

        // rowid 1
        let cell_bytes: Vec<u8> = vec![
            27, 1, 4, 0, 37, 35, 71, 114, 97, 110, 110, 121, 32, 83, 109, 105, 116, 104, 76, 105,
            103, 104, 116, 32, 71, 114, 101, 101, 110,
        ];
        let cell = LeafTableCell::parse(cell_bytes.as_slice()).unwrap();
        assert_eq!(cell.rowid, 1);
        assert_eq!(cell.payload.values.len(), 3);
        assert_eq!(cell.payload.values[0], ColumnValue::Null);
        assert_eq!(
            cell.payload.values[1],
            ColumnValue::Text("Granny Smith".to_owned())
        );
        assert_eq!(
            cell.payload.values[2],
            ColumnValue::Text("Light Green".to_owned())
        );
    }

    #[test]
    fn test_parse_sqlite_schema_sample_db() {
        let cell_bytes: Vec<u8> = vec![
            120, 3, 7, 23, 27, 27, 1, 129, 71, 116, 97, 98, 108, 101, 111, 114, 97, 110, 103, 101,
            115, 111, 114, 97, 110, 103, 101, 115, 4, 67, 82, 69, 65, 84, 69, 32, 84, 65, 66, 76,
            69, 32, 111, 114, 97, 110, 103, 101, 115, 10, 40, 10, 9, 105, 100, 32, 105, 110, 116,
            101, 103, 101, 114, 32, 112, 114, 105, 109, 97, 114, 121, 32, 107, 101, 121, 32, 97,
            117, 116, 111, 105, 110, 99, 114, 101, 109, 101, 110, 116, 44, 10, 9, 110, 97, 109,
            101, 32, 116, 101, 120, 116, 44, 10, 9, 100, 101, 115, 99, 114, 105, 112, 116, 105,
            111, 110, 32, 116, 101, 120, 116, 10, 41,
        ];
        let cell = LeafTableCell::parse(cell_bytes.as_slice()).unwrap();

        info!("{cell:?}");
        assert_eq!(cell.rowid, 3);
        assert_eq!(cell.payload.values.len(), 5); // schema object row has 5 values
                                                  // db obj type
        assert_eq!(
            cell.payload.values[0],
            ColumnValue::Text("table".to_owned())
        );
        // name
        assert_eq!(
            cell.payload.values[1],
            ColumnValue::Text("oranges".to_owned())
        );
        // tbl_name
        assert_eq!(
            cell.payload.values[2],
            ColumnValue::Text("oranges".to_owned())
        );
        // rootpage
        assert_eq!(cell.payload.values[3], ColumnValue::int8(4));
        // sql
        assert_eq!(cell.payload.values[4], ColumnValue::Text("CREATE TABLE oranges\n(\n\tid integer primary key autoincrement,\n\tname text,\n\tdescription text\n)".to_owned()));
    }
}

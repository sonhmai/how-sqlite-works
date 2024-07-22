use anyhow::Result;

use crate::model::cell_table_leaf::LeafTableCell;
use crate::model::db_header::DbHeader;
use crate::model::page::Page;
use crate::model::schema::SchemaObject;

/// DbMeta holds meta information of the database
/// - db header
/// - db schema: database object schema's (table, index, etc.)
///
#[derive(Debug)]
pub struct DbMeta {
    pub db_header: DbHeader,
    pub schema_objects: Vec<SchemaObject>, // table, index, view,...
}

impl DbMeta {
    pub fn parse(db: &[u8]) -> Result<Self> {
        let db_header = DbHeader::parse(&db[..DbHeader::SIZE])?;
        let page_size: usize = db_header.page_size.try_into()?;
        let mut first_page = Page::parse_db_schema_page(db, page_size)?;
        let page_content_offset = first_page.page_header.content_start_offset;

        let leaf_table_cells: Vec<LeafTableCell> = first_page
            .cell_ptrs()
            .iter()
            // use &cell_ptr to borrow usize value
            .map(|&cell_ptr| LeafTableCell::parse(&db[cell_ptr..]).unwrap())
            .collect();

        let schema_objects: Vec<SchemaObject> = leaf_table_cells
            .iter()
            .map(|cell| SchemaObject::parse(cell))
            // filter out SchemaObject "sqlite_sequence" because cannot parse
            // DDL without datatype yet: CREATE TABLE sqlite_sequence(name,seq)
            .flatten()
            .collect();

        Ok(DbMeta {
            db_header,
            schema_objects,
        })
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use crate::model::db_meta::DbMeta;
    use crate::test_utils::db_bytes;

    #[test]
    fn test_parse_db_meta() {
        let db_meta = DbMeta::parse(db_bytes().as_slice()).unwrap();
        info!("{db_meta:?}")
    }
}

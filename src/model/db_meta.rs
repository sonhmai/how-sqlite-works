use crate::model::cell_table_leaf::LeafTableCell;
use anyhow::Result;

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
            .map(|cell| SchemaObject::parse(&cell))
            // filter out SchemaObject "sqlite_sequence" because cannot parse
            // DDL without datatype yet: CREATE TABLE sqlite_sequence(name,seq)
            .filter(|result| result.is_ok())
            .map(|result| result.unwrap())
            .collect();

        Ok(DbMeta {
            db_header,
            schema_objects,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::db_meta::DbMeta;
    use log::info;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_parse_db_meta() {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources/sample.db");
        let data = fs::read(db_path).unwrap();
        let db_slice = data.as_slice();

        let db_meta = DbMeta::parse(db_slice).unwrap();
        info!("{db_meta:?}")
    }
}

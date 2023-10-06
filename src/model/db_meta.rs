use anyhow::Result;

use crate::model::db_header::DbHeader;

/// DbMeta holds meta information of the database
/// - db header
/// - db schema: database object schema's (table, index, etc.)
///
#[derive(Debug)]
pub struct DbMeta {
    db_header: DbHeader,
    // schema_objects: Vec<SchemaObject>, // table, index, view,...
}

impl DbMeta {
    pub fn parse(db: &[u8]) -> Result<Self> {
        let db_header = DbHeader::parse(&db[..DbHeader::SIZE])?;


        Ok(DbMeta {
            db_header,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::model::db_meta::DbMeta;

    #[test]
    fn test_parse_db_meta() {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let data = fs::read(db_path).unwrap();
        let db_slice = data.as_slice();

        let db_meta = DbMeta::parse(db_slice).unwrap();
        println!("{db_meta:?}")
    }
}
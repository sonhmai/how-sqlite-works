use anyhow::Result;

use crate::model::cell::LeafTableCell;

/// Schema Table https://www.sqlite.org/schematab.html
/// https://www.sqlite.org/fileformat.html#storage_of_the_sql_database_schema
///
/// Table looks like this
///
/// CREATE TABLE sqlite_schema(
///   type text,
///   name text,
///   tbl_name text,
///   rootpage integer,
///   sql text
/// );
///
/// sqlite_schema table contains one row for each table, index, view, and trigger
/// (collectively "objects") in the database schema, except there is no entry
/// for the sqlite_schema table itself.
/// The sqlite_schema table contains entries for internal schema objects
/// in addition to application- and programmer-defined objects.
#[derive(Debug, PartialEq)]
pub enum SchemaObjType {
    Table,
    Index,
    View,
    Trigger,
}

#[derive(Debug)]
pub struct SchemaObject {
    pub obj_type: SchemaObjType,
    pub name: String,
    pub tbl_name: String,
    // The maximum page number is 4,294,967,294 (2^32 - 2): use u32 not i32
    // https://www.sqlite.org/fileformat.html#pages
    pub rootpage: u32,
    pub sql: String,
}

impl SchemaObject {
    pub fn parse(cell: &LeafTableCell) -> Result<Self> {
        let obj_type = SchemaObjType::Table; // TODO fix hard-coded table here, include other types
        let name = String::try_from(cell.payload.value_at_index(1))?;
        let tbl_name = String::try_from(cell.payload.value_at_index(2))?;
        let rootpage = i32::try_from(cell.payload.value_at_index(3))?;
        let sql = String::try_from(cell.payload.value_at_index(4))?;

        Ok(Self {
            obj_type,
            name,
            tbl_name,
            rootpage: rootpage as u32,
            sql,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::column_value::ColumnValue;

    use super::*;

    #[test]
    fn test_parse_schema_object_from_cell_sample_db() {
        let cell_bytes: Vec<u8> = vec![120, 3, 7, 23, 27, 27, 1, 129, 71, 116, 97, 98, 108, 101, 111, 114, 97, 110, 103, 101, 115, 111, 114, 97, 110, 103, 101, 115, 4, 67, 82, 69, 65, 84, 69, 32, 84, 65, 66, 76, 69, 32, 111, 114, 97, 110, 103, 101, 115, 10, 40, 10, 9, 105, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 97, 114, 121, 32, 107, 101, 121, 32, 97, 117, 116, 111, 105, 110, 99, 114, 101, 109, 101, 110, 116, 44, 10, 9, 110, 97, 109, 101, 32, 116, 101, 120, 116, 44, 10, 9, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 116, 101, 120, 116, 10, 41];

        let cell = LeafTableCell::parse(cell_bytes.as_slice()).unwrap();
        assert_eq!(cell.rowid, 3);
        assert_eq!(cell.payload.values.len(), 5); // schema object row has 5 values
        assert_eq!(cell.payload.values[0], ColumnValue::Text("table".to_owned()));
        assert_eq!(cell.payload.values[1], ColumnValue::Text("oranges".to_owned()));
        assert_eq!(cell.payload.values[2], ColumnValue::Text("oranges".to_owned()));
        assert_eq!(cell.payload.values[3], ColumnValue::int8(4));
        assert_eq!(cell.payload.values[4], ColumnValue::Text("CREATE TABLE oranges\n(\n\tid integer primary key autoincrement,\n\tname text,\n\tdescription text\n)".to_owned()));

        let schema_obj = SchemaObject::parse(&cell).unwrap();

        assert_eq!(schema_obj.obj_type, SchemaObjType::Table);
        assert_eq!(schema_obj.name, "oranges".to_owned());
        assert_eq!(schema_obj.tbl_name, "oranges".to_owned());
        assert_eq!(schema_obj.rootpage, 4);
        assert_eq!(schema_obj.sql, "CREATE TABLE oranges\n(\n\tid integer primary key autoincrement,\n\tname text,\n\tdescription text\n)".to_owned());
    }
}

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
#[derive(Debug)]
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
    pub rootpage: u32,
    pub sql: String,
}


use anyhow::{bail, Result};

use arrow_schema::{DataType, Field, Schema};
use datafusion_sql::sqlparser;
use datafusion_sql::sqlparser::ast::Statement::CreateTable;
use datafusion_sql::sqlparser::dialect::SQLiteDialect;
use datafusion_sql::sqlparser::parser::Parser;

pub fn parse_columns_from_ddl(ddl: &str) -> Result<Vec<Field>> {
    let dialect = SQLiteDialect {};
    let statements = Parser::parse_sql(&dialect, ddl)?;

    if statements.len() != 1 {
        bail!("Invalid DDL statement".to_string())
    }

    if let CreateTable { columns, .. } = &statements[0] {
        let fields: Result<Vec<Field>> = columns
            .iter()
            .map(|column_def| {
                let data_type = match &column_def.data_type {
                    sqlparser::ast::DataType::Varchar(_) => DataType::Utf8,
                    sqlparser::ast::DataType::Text => DataType::Utf8,
                    sqlparser::ast::DataType::Int(_) => DataType::Int32,
                    sqlparser::ast::DataType::Integer(_) => DataType::Int32,
                    // Add more mappings for other data types as needed
                    _ => {
                        bail!(format!("Unsupported data type: {}", column_def.data_type));
                    }
                };

                Ok(Field::new(&column_def.name.value, data_type, false))
            })
            .collect();

        fields
    } else {
        bail!("Invalid DDL statement".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_key_auto_increment() {
        let ddl = "\
            CREATE TABLE oranges(\
                id integer primary key autoincrement,\
                name text,\
                description text\
        )";
        let fields = parse_columns_from_ddl(ddl).unwrap();

        assert_eq!(fields.len(), 3);

        assert_eq!(fields[0].name(), "id");
        assert_eq!(fields[0].data_type(), &DataType::Int32);

        assert_eq!(fields[1].name(), "name");
        assert_eq!(fields[1].data_type(), &DataType::Utf8);

        assert_eq!(fields[2].name(), "description");
        assert_eq!(fields[2].data_type(), &DataType::Utf8);
    }

    #[ignore = "sqlparser cannot parse SQLite statement without data type"]
    #[test]
    fn test_parse_columns_from_ddl() {
        let ddl = "CREATE TABLE sqlite_sequence(name,seq)";
        // TODO - handle this sqlparser cannot parse SQLite statement without data type
        let fields = parse_columns_from_ddl(ddl).unwrap();

        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_parse_columns_from_ddl_invalid() {
        let ddl = "CREATE TABLE my_table (id INT, name VARCHAR(50), age";
        let result = parse_columns_from_ddl(ddl);

        assert!(result.is_err());
    }
}

fn main() {
    let ddl = "CREATE TABLE my_table (id INT, name VARCHAR(50), age INT)";

    match parse_columns_from_ddl(ddl) {
        Ok(fields) => {
            let schema = Schema::new(fields);
            println!("Parsed columns and data types:");
            for field in schema.fields() {
                println!("{}: {:?}", field.name(), field.data_type());
            }
        }
        Err(err) => eprintln!("Error parsing DDL: {}", err),
    }
}

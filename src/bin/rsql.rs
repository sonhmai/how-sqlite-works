use clap::{App, Arg, SubCommand};
use datafusion_sql::{
    sqlparser::{dialect::SQLiteDialect, parser::Parser}
};
use datafusion_sql::planner::SqlToRel;
use datafusion_sql::sqlparser::ast::Statement;
use datafusion_sql::sqlparser::dialect::AnsiDialect;

use rsql::model::database::Database;
use rsql::model::table::Table;
use rsql::sql::schema_provider::SchemaProvider;

fn main() {
    let matches = App::new("rust-sqlite")
        .subcommand(
            SubCommand::with_name("sql")
                .about("Execute sql query against a db file")
                .arg(Arg::with_name("db_file_path").required(true))
                .arg(Arg::with_name("sql").help("SQL string to execute").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("sql", Some(_matches)) => {
            let db = _matches.value_of("db_file_path").unwrap();
            let sqlstr = _matches.value_of("sql").unwrap();
            let table = Table {
                name: "hardcoded".to_string()
            };
            let db = Database::new(db);
            println!("Executing '{sqlstr}' against db {db:?}, table {table:?}");

            // sql to unoptimized logical plan
            let dialect = AnsiDialect {};
            let ast: Vec<Statement> = Parser::parse_sql(&dialect, sqlstr).unwrap();
            let statement = &ast[0];
            // create logical query plan
            let schema_provider = SchemaProvider::new();
            let sql_to_rel = SqlToRel::new(&schema_provider);
            let plan = sql_to_rel.sql_statement_to_plan(statement.clone()).unwrap();
            println!("{plan:?}");

        }
        _ => unreachable!()
    }
}

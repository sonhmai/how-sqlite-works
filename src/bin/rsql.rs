use clap::{App, Arg, SubCommand};
use datafusion_sql::planner::SqlToRel;
use datafusion_sql::sqlparser::parser::Parser;
use datafusion_sql::sqlparser::ast::Statement;
use datafusion_sql::sqlparser::dialect::AnsiDialect;

use log::info;
use rsql::model::data_record::DataRecord;
use rsql::model::database::Database;
use rsql::physical::physical_planner::PhysicalPlanner;
use rsql::sql::context_provider::SqliteContextProvider;

fn main() {
    env_logger::init();
    let matches = App::new("rust-sqlite")
        .subcommand(
            SubCommand::with_name("sql")
                .about("Execute sql query against a db file")
                .arg(Arg::with_name("db_file_path").required(true))
                .arg(
                    Arg::with_name("sql")
                        .help("SQL string to execute")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("sql", Some(_matches)) => {
            let db_file_path = _matches.value_of("db_file_path").unwrap();
            let sqlstr = _matches.value_of("sql").unwrap();
            let db = Database::new(db_file_path).unwrap();
            info!("Executing '{sqlstr}' against db {db_file_path}");

            // sql to unoptimized logical plan
            let dialect = AnsiDialect {};
            let ast: Vec<Statement> = Parser::parse_sql(&dialect, sqlstr).unwrap();
            let statement = &ast[0];
            // create logical query plan
            let schema_provider = SqliteContextProvider::new_for_db(&db);
            let sql_to_rel = SqlToRel::new(&schema_provider);
            let logical_plan = sql_to_rel.sql_statement_to_plan(statement.clone()).unwrap();
            let physical_planner = PhysicalPlanner {};
            let exec = physical_planner.plan(&logical_plan);
            info!("Physical plan: {exec:?}");
            let records = exec.execute();
            info!("Returned records: {records:?}");
            sqlite_show(&records);
        }
        _ => unreachable!(),
    }
}

/// show CLI output in SQLite format
fn sqlite_show(records: & Vec<DataRecord>) {
    for record in records {
        let formatted_values: Vec<String> = record
            .values
            .iter()
            .map(|value| value.to_string())
            .collect();

        let output = formatted_values.join("|");
        println!("{}", output);
    }
}
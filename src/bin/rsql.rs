use clap::{App, Arg, SubCommand};
use datafusion_sql::planner::SqlToRel;
use datafusion_sql::sqlparser::ast::Statement;
use datafusion_sql::sqlparser::dialect::AnsiDialect;
use datafusion_sql::sqlparser::parser::Parser;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use log::info;
use rsql::model::database::Database;
use rsql::physical::physical_planner::PhysicalPlanner;
use rsql::sql::context_provider::SqliteContextProvider;
use rsql::util::presentation;

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
            let db_ref = Rc::new(RefCell::new(db));
            let physical_planner = PhysicalPlanner {
                database: db_ref.clone(),
            };
            let mut ref_plan = physical_planner.plan(&logical_plan);
            // get a mutable ref to execute Exec, need to be sure no other refs to Arc
            let exec = Arc::get_mut(&mut ref_plan).unwrap();
            info!("Physical plan: {exec:?}");
            let records = exec.execute();
            info!("Returned records: {records:?}");
            presentation::sqlite_show(records);
        }
        _ => unreachable!(),
    }
}

use clap::{App, Arg, SubCommand};
use rsql::model::database::Database;

use rsql::model::table::Table;

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
            println!("Executing {sqlstr} against db {:?}, table {:?}", db, table)
        }
        _ => unreachable!()
    }
}

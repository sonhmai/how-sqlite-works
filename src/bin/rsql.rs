use clap::{App, Arg, SubCommand};

fn main() {

    let matches = App::new("rust-sqlite")
        .subcommand(
            SubCommand::with_name("sql")
                .about("Execute sql query against a db file")
                .arg(Arg::with_name("db").required(true))
                .arg(Arg::with_name("sql").help("SQL string to execute").required(true))
        )
        .get_matches();

    match matches.subcommand() {
        ("sql", Some(_matches)) => {
            let db = _matches.value_of("db").unwrap();
            let sqlstr = _matches.value_of("sql").unwrap();
            println!("Executing {sqlstr} against {db}")
        }
        _ => unreachable!()
    }
}

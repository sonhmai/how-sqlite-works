use assert_cmd::prelude::*;
use std::process::Command;

/*
❯ sqlite3 tests/resources/sample.db ".schema"

CREATE TABLE apples
(
    id integer primary key autoincrement,
    name text,
    color text
);
CREATE TABLE sqlite_sequence(name,seq);
CREATE TABLE oranges
(
    id integer primary key autoincrement,
    name text,
    description text
);

DataFusion logical plan
    Projection: apples.id, apples.name, apples.color, oranges.id, oranges.name, oranges.description
      Inner Join:  Filter: apples.id = oranges.id
        TableScan: apples
        TableScan: oranges
*/
#[test]
fn cli_sql_inner_join_two_tables() {
    Command::cargo_bin("rsql")
        .unwrap()
        .args([
            "sql",
            "tests/resources/sample.db",
            "select * from apples join oranges on apples.id=oranges.id;",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            r#"1|Granny Smith|Light Green|1|Navel|Orange
2|Fuji|Red|2|Blood Orange|Deep Red
3|Honeycrisp|Blush Red|3|Cara Cara|Pinkish Red
4|Golden Delicious|Yellow|4|Seville|Bitter Orange"#,
        ));
}

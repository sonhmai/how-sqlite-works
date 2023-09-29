
use assert_cmd::prelude::*;
use predicates::ord::eq;
use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("rsql").unwrap().assert().failure();
}

#[ignore] // feature not implemented yet
#[test]
fn cli_dbinfo() {
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&["sample.db", ".dbinfo"])
        .assert()
        .success()
        .stdout(eq("database page size: 4096"));
}


#[test]
fn cli_sql_group_by_count() {
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&["sql", "sample.db", "select color, count(color) from apples group by color;"])
        .assert()
        .success();
}
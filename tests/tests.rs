use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("rsql").unwrap().assert().failure();
}

#[test]
fn cli_sql_group_by_count() {
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&["sql", "sample.db", "select color, count(color) from apples group by color;"])
        .assert()
        .success();
}
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn cli_no_args() {
    Command::cargo_bin("rsql").unwrap().assert().failure();
}

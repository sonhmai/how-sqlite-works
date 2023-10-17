use assert_cmd::prelude::*;
use predicates::ord::eq;
use std::process::Command;

#[ignore]
#[test]
fn cli_sql_scan_table_single_page() {
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&[
            "sql",
            "tests/resources/sample.db",
            "select * from apples;",
        ])
        .assert()
        .success()
        .stdout(eq(r#"1|Granny Smith|Light Green
2|Fuji|Red
3|Honeycrisp|Blush Red
4|Golden Delicious|Yellow"#));
}

#[ignore]
#[test]
fn cli_sql_scan_table_single_page_projection() {
    // subset columns
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&[
            "sql",
            "tests/resources/sample.db",
            "select name from apples;",
        ])
        .assert()
        .success()
        .stdout(eq(r#"Granny Smith
Fuji
Honeycrisp
Golden Delicious"#));
}

#[ignore]
#[test]
fn cli_sql_scan_table_multiple_pages() {
    // Traversing only the first table page is not enough to pass this test
    Command::cargo_bin("rsql")
        .unwrap()
        .args(&[
            "sql",
            "tests/resources/superheroes.db",
            "SELECT id, name FROM superheroes WHERE eye_color = 'Pink Eyes';",
        ])
        .assert()
        .success()
        .stdout(eq(r#"297|Stealth (New Earth)
790|Tobias Whale (New Earth)
1085|Felicity (New Earth)
2729|Thrust (New Earth)
3289|Angora Lapin (New Earth)
3913|Matris Ater Clementia (New Earth)"#));
}

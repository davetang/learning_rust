use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    // run binary with hello as an argument and
    // verify that it exits successfully
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() {
    let outfile = "tests/expected/hello1.txt";
    // read contents of file; only use this function with small files
    let expected = fs::read_to_string(outfile).unwrap();
    // Create a Command to run echor
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

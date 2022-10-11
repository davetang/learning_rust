// [dev-dependencies]
// assert_cmd = "2"
// predicates = "2"
use assert_cmd::Command;
use predicates::prelude::*;

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

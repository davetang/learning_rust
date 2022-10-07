use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn check_output() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

// import from the standard library
use std::process::Command;

#[test]
fn runs() {
    // let will bind a value to a variable
    // mut makes the variable mutable
    // by default, Rust variables are immutable
    let mut cmd = Command::new("ls");
    // capture output as a Result
    let res = cmd.output();
    // verify that the result is an Ok variant
    assert!(res.is_ok());
}

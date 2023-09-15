use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

/*

    Create a type alias called TestResult

this will be a specific type of Result that is either an Ok that always contains the unit type or
some value that implements the std::error::Error trait

Box indicates that the error will live inside a kind of pointer where the memory is dynamically
allocated on the heap rather than the stack, and dyn indicates that the method calls on the
std::error::Error trait are dynamically dispatched

the Ok part of TestResult will only ever hold the unit type
the Err part can hold anything that implements the std::error::Error trait

*/
type TestResult = Result<(), Box<dyn std::error::Error>>;

// the ? operator will either unpack an Ok value or propagate the Err value to the return type
// this will cause the function to return the Err variant of Option to the caller, which will in
// turn cause the test to fail
// In Rust, omit the semicolon from the last expression to implicitly return that result
// --------------------------------------------------
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

// --------------------------------------------------
#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

// --------------------------------------------------
#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

// --------------------------------------------------
#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

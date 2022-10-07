# Learning Rust

Because why not!?

## Docker

Use the [official Docker image](https://hub.docker.com/_/rust/).

```bash
docker pull rust:1.64.0

docker run --rm rust:1.64.0 cat /etc/os-release
PRETTY_NAME="Debian GNU/Linux 11 (bullseye)"
NAME="Debian GNU/Linux"
VERSION_ID="11"
VERSION="11 (bullseye)"
VERSION_CODENAME=bullseye
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"
```

Use `script/start_container.sh` to start and restart a container using
rust:1.64.0 called `learning_rust`.

## Getting started

The Rust compiler is `rustc`. Compile the hello world example by running:

```bash
rustc eg/hello.rs
```

Cargo is Rust's build tool, package manager, and test runner. You can use it to
start a new Rust project.

```bash
cargo new hello
```

`Cargo.toml` is a configuration file for the project and stands for [Tom's
Obvious, Minimal Language](https://en.wikipedia.org/wiki/TOML). The `src`
directory is for source code files and `main.rs` is the default starting point.

Use `cargo run` (in the `hello` directory) to compile and run or else you'll
get the following error `error: could not find Cargo.toml in /work or any
parent directory`.

```bash
cargo run
```

By default, Cargo will build a `debug` target and there will be a
`target/debug` directory containing the build artifacts.

### Testing

Inside-out or unit testing is when you write tests for the functions inside
your program. Outside-in or integration testing is when you write tests that
run your programs. The convention in Rust projects is to create a `tests`
directory in the same level as `src`.

Save the code below in a file inside `tests` and run `cargo test`.

```rust
#[test]
fn works() {
    assert!(true);
}
```

The `#[test]` attribute tells Rust to run this function when testing and the
`assert!` macro asserts that a Boolean expression is true. There is another
macro called `assert_eq!` that is used to verify that something is an expected
value.

Below is another test that executes a command and checks the result.

```rust
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
    // verify that the result is an OK variant
    assert!(res.is_ok());
}
```

### Adding a project dependency

Add the following lines to `Cargo.toml`. The `dev-dependencies` indicates that
this crate is only used for testing and benchmarking.

```
[dev-dependencies]
assert_cmd = "1"
```

## Notes

* Rust libraries are called crates and they are expected to use semantic version
numbers in the form major.minor.patch.


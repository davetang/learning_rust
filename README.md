Table of Contents
=================

* [Learning Rust](#learning-rust)
   * [Installation](#installation)
   * [Getting started](#getting-started)
      * [Testing](#testing)
      * [Adding a project dependency](#adding-a-project-dependency)
   * [Useful crates](#useful-crates)
   * [Notes](#notes)
   * [Resources](#resources)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->

# Learning Rust

Because why not!?

## Installation

Use `rustup`, a command line tool for managing Rust versions and associated
tools, to [install
Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

```console
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# 1) Proceed with installation (default)
# Rust is installed now. Great!

rustc --version
# rustc 1.72.0 (5680fa18f 2023-08-23)
```

Updating.

```console
rustup update
```

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
parent directory`. Use `cargo build` for just building and not running.

```bash
cargo run
```

By default, Cargo will build a `debug` target and there will be a
`target/debug` directory containing the build artifacts. Use `cargo clean` to
remove the target directory.

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

## Useful crates

* For parsing command-line arguments, use [clap](https://crates.io/crates/clap) (command-line argument parser).

## Notes

* Rust libraries are called crates and they are expected to use semantic
  version numbers in the form major.minor.patch.
* A _trait_ in Rust is a way to define the behaviour of an object in an
  abstract way. For example, if an object implements the `Display` trait, then
  it can be formatted for user-facing output.

## Resources

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust-Bio: a fast and safe bioinformatics library](https://academic.oup.com/bioinformatics/article/32/3/444/1743419)
> "The key feature of Rust is a concept of ownership and borrowing of
variables, that enables the compiler to automatically decide about lifetime of
objects during compile time, making an online memory management superfluous
without requiring manual freeing of resources. At the same time, this concept
prevents common sources of errors with low-level languages like accessing
invalid memory regions. Finally, the ownership concept enforces thread-safety,
such that race conditions cannot occur. These features make Rust a promising
solution to above tradeoff problem."

* [freeCodeCamp's complete Rust programming
course](https://www.freecodecamp.org/news/rust-programming-course-for-beginners/)
(14-hour watch).

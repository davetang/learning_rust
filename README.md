Table of Contents
=================

* [Learning Rust](#learning-rust)
   * [Installation](#installation)
   * [Docker](#docker)
   * [Getting started](#getting-started)
      * [Testing](#testing)
      * [Adding a project dependency](#adding-a-project-dependency)
   * [Cargo](#cargo)
   * [Useful crates](#useful-crates)
   * [Notes](#notes)
   * [Resources](#resources)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->

# Learning Rust

Because why not!?

Rust is syntactically similar to C, so there are `for` loops, semicolon
terminated statements, and blocks formed by curly braces. Rust is statically
typed, meaning that a variable can never change its type but you don't always
have to declare a variable's type in Rust because the compiler can often figure
it out from the context. Variables in Rust are _immutable_ by default.
Functions are _first-class_ values, like in functional programming languages.

Rust is not an object-oriented language, as there are no classes or inheritance
in Rust. Instead, Rust uses a `struct` (structure) to represent complex data
types and _traits_ to describe how types can behave. These structures can have
methods, can mutate the internal state of the data, and might even be called
_objects_ in the documentation, but they are not objects in the formal sense of
the word.

Rust can guarantee memory safety through the use of a _borrow checker_ that
tracks which part of a program has safe access to different parts of memory and
this safety does not come at the expense of performance.

Rust programs compile to native binaries and often match or beat the speed of
programs written in C or C++. For these reasons, Rust is often described as a
systems programming language that has been designed for performance and safety.

## Installation

Rust can be installed using `rustup`, which is a command line tool for managing
Rust versions and associated tools. Open a terminal, enter the command below
and follow the prompt.

```console
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
```
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```
```console
source "$HOME/.cargo/env"
rustc --version
```
```
rustc 1.71.1 (eb26296b5 2023-08-03)
```

To update.

```console
rustup update
```

To uninstall.

```console
rustup self uninstall
```

## Docker

Use the [official Docker image](https://hub.docker.com/_/rust/).

```console
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

```console
rustc eg/hello.rs

./hello
# Hello, world!
```

Determine file type.

```console
file ./hello

# ./hello: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked (uses shared libs), for GNU/Linux 2.6.32, BuildID[sha1]=81ba1f0be3c7a8c64dfc5e3e05959affe202f9a3, not stripped
```

Cargo is Rust's build tool, package manager, and test runner. You can use it to
start a new Rust project. The `src` directory is for source code files and
`main.rs` is the default starting point.

```console
cargo new new_project
#      Created binary (application) `new_project` package

tree --charset ascii new_project/
# new_project/
# |-- Cargo.toml
# `-- src
#     `-- main.rs
#
# 1 directory, 2 files
```

`Cargo.toml` is a configuration file for the project and stands for [Tom's
Obvious, Minimal Language](https://en.wikipedia.org/wiki/TOML).

```toml
[package]
name = "new_project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The `edition` key is the edition of Rust that should be used to compile the
program. Editions are how the Rust community introduces changes that are not
backward compatible.

Use `cargo run` (in the project root directory) to compile and run or else
you'll get the following error `error: could not find Cargo.toml in /work or
any parent directory`. Use `cargo build` for just building and not running.

```console
cd new_project
cargo run
#    Compiling new_project v0.1.0 (/home/dtang/github/learning_rust/new_project)
#     Finished dev [unoptimized + debuginfo] target(s) in 0.20s
#      Running `target/debug/new_project`
# Hello, world!
```

By default, Cargo will build a `debug` target and there will be a
`target/debug` directory containing the build artifacts. Use `cargo clean` to
remove the target directory.

### Getting help

You can get help on any of Cargo's commands using:

    cargo help command

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

## Cargo

Cargo commands.

* Create a project using `cargo new`.
* Build a project using `cargo build`.
* Build and run a project using `cargo run`.
* Build a project without producing a binary to check for errors using `cargo
  check`; this is faster than `cargo build` and is useful for testing.
* Run tests inside `tests` using `cargo test`
* Compile with optimisations using `cargo build --release`; this executable is
  much faster but takes longer to compile so use it only when you are ready for
  release.
* To remove the target directory, use `cargo clean`.

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
* [Rust-bio](https://docs.rs/bio/latest/bio/), a bioinformatics library for
Rust

# README

* Process command-line arguments with the `clap` crate

We can use `std::env::args` but `clap` provides more functionality and is
simple to use.

```rust
// import the App and Arg structs from the clap crate
use clap::{App, Arg};

// positional argument named text
// optional flag named omit_newline
fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Dave Tang <me@davetang.org>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .long("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches()
}
```

`App::get_matches` will display an error and exit with an appropriate error
code is parsing failed.

The `_` prefix for `matches` tells the Rust compiler that we do not intend to
use the variable right now. Without the underscore, the compiler would warns us
about an unused variable.

* Use Rust types like strings, vectors, slices, and the unit type

Rust vectors must all be of the same type; literal strings must be enclosed in
double quotes. The `str` type in Rust represents a valid UTF-8 string.

* Use expressions like `match`, `if`, and `return`

In Rust, `if` is an expression, not a statement; an expression returns a value
but a statement does not.

An `if` without an `else` will return the unit type.

* Use `Option` to represent an optional value
* Handle errors using the `Result` variants of `Ok` and `Err`

It is common practice in Rust to present facts using `assert!` to say that
something is `true` and `assert_eq!` to demonstrate that one thing is
equivalent to another.

* Understand the difference between stack and heap memory

A stack is where items of known sizes are accessed in a particular order. The
  classic analogy is to a stack of cafeteria trays where new items go on top
  and are taken back off the top in last-in, first-out (LIFO) order. Items on
  the stack have a fixed, known size, making it possible for Rust to set aside
  a particular chunk of memory and find it quickly.

A heap is where the sizes of the values may change over time. For example, the
  documentation for the Vec (vector) type describes this structure as a
  "contiguous growable array type." The number and sizes of the elements in a
  vector can change during the lifetime of the program. Rust makes an initial
  estimation of the amount of memory needed. If the vector grows beyond the
  original allocation, Rust will find another chunk of memory to hold the data.
  To find the memory where the data resides, Rust stores the memory address on
  the stack. This is called a pointer because it points to the actual data, and
  so it also said to be a reference to the data.

* Test for text that is printed to `STDOUT` and `STDERR`

## Testing

Running `cargo test dies` will run all the tests with names containing the
string "dies".

```rust
use assert_cmd::Command;
use predicates::prelude::*;

// Run `echor` with no arguments and assert that it fails and prints a usage
// statement to STDERR.
#[test]
fn dies_no_args() {
    let mut cmd = Commnad::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}
```

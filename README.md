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
get the following error `error: could not find `Cargo.toml` in `/work` or any
parent directory`.

```bash
cargo run
```

By default, Cargo will build a `debug` target and there will be a
`target/debug` directory containing the build artifacts.

## Notes

* Rust libraries are called crates and they are expected to use semantic version
numbers in the form major.minor.patch.


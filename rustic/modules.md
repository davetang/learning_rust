## Table of Contents

  - [Modules](#modules)
  - [Packages and Crates](#packages-and-crates)

## Modules

Rust has a number of features that allow you to manage your code's
organisation, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the _module system_, include:

* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules** and **use**: Let you control the organisation, scope, and privacy
  of paths
* **Paths**: A way of naming an item, such as a struct, function, or module

## Packages and Crates

A `crate` is the smallest amount of code that the Rust compiler considers at a
  time. Even if you run `rustc` rather than `cargo` and pass a single source
  code file, the compiler considers that file to be a crate. Crates can contain
  modules, and the modules may be defined in other files that get compiled with
  the crate.

A crate can come in one of two forms: a binary crate or a library crate.
  _Binary crates_ are programs you can compile to an executable that you can
  run. Each must have a function called `main` that defines what happens when
  the executable runs.

_Library crates_ don't have a `main` function and they don't compile to an
executable. Instead, they define functionality intended to be shared with
multiple projects. Most of the time when Rust programmers say "crate", they
mean library crate, and they use "crate" interchangeably with the general
programming concept of a "library".

The _crate root_ is a source file that the Rust compiler starts from and makes
up the root module of your crate.

A _package_ is a bundle of one or more crates that provides a set of
  functionality. A package contains a _Cargo.toml_ file that describes how to
  build those crates. Cargo is actually a package that contains the binary
  crate for the command-line tool used to build Rust code. The Cargo package
  also contains a library crate that the binary crate depends on. Other
  projects can depend on the Cargo library crate to use the same logic the
  Cargo command-line tool uses.

A package can contain as many binary crates as you like (by placing files in
  _src/bin_), but only one library crate. A package must contain at least one
  crate, whether that's a library or binary crate.

When we create a package using the `cargo new` command, a _Cargo.toml_ file is
created giving us a package. A _src_ directory is also created with a _main.rs_
file. If you open _Cargo.tom_, you'll see that there's no mention of
_src/main.rs_. Cargo follows a convention that _src/main.rs_ is the crate root
of a binary crate with the same name as the package. Likewise, Cargo knows that
if the package directory contains _src/lib.rs_, the package contains a library
  crate with the same name as the package, and _src/lib.rs_ is its crate root.
  Cargo passes the crate root files to `rustc` to build the library or binary.

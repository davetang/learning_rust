## Table of Contents

  - [Modules](#modules)
  - [Packages and Crates](#packages-and-crates)
  - [Modules Cheat Sheet](#modules-cheat-sheet)
  - [Grouping Related Code in Modules](#grouping-related-code-in-modules)

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

## Modules Cheat Sheet

Here's a quick reference on how modules, paths, the `use` keyword, and the
`pub` keyword work in the compiler and how most developers organise their code.

* **Start from the crate root**: When compiling a crate, the compiler first
looks in the crate root file (usually _src/lib.rs_ for a library crate or
_src/main.rs_ for a binary crate) for code to compile.

* **Declaring modules**: In the crate root file, you can declare new modules;
for example a "garden" module is declared with `mod garden;`. The compiler will
  look for the module's code in:

    * Inline, within curly brackets that replace the semicolon following `mod garden`
    * In the file _src/garden.rs_
    * In the file _src/garden/mod.rs_

* **Declaring submodules**: In any file other than the crate root, you can
declare submodules. For example, you might declare `mod vegetables;` in
_src/garden.rs_. The compiler will look for the submodule's code within the
directory named for the parent module in these places:

    * Inline, directly following `mod vegetables`, within curly brackets
      instead of the semicolon
    * In the file _src/garden/vegetables.rs_
    * In the file _src/garden/vegetables/mod.rs_

* **Paths to code in modules**: Once a module is part of your crate, you can
refer to code in that module from anywhere else in that same crate, as long as
the privacy rules allow, using the path to the code. For example, an
`Asparagus` type in the garden vegetables module would be found at
`crate::garden::vegetables::Asparagus`.

* **Private vs public**: Code within a module is private from its parent
modules by default. To make a module public, declare it with `pub mod` instead
of `mod`. To make items within a public module public as well, use `pub` before
their declarations.

* **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to
items to reduce repetition of long paths. In any scope that can refer to
`crate::garden::vegetables::Asparagus`, you can create a shortcut with `use
crate::garden::vegetables::Asparagus;` and from then on you only need to write
`Asparagus` to make use of that type in the scope.

The binary crate below named `backyard` illustrates the above rules. The
crate's directory, also named backyard, contains these files and directories:

```
backyard
|-- Cargo.lock
|-- Cargo.toml
`-- src
    |-- garden
    |   `-- vegetables.rs
    |-- garden.rs
    `-- main.rs
```

The crate root in this case is _src/main.rs_, and it contains:

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in
_src/garden.rs_, which is:

```rust
pub mod vegetables;
```

Here, `pub mod vegetables;` means the code in _src/garden/vegetables.rs_ is
included too. That code is:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

## Grouping Related Code in Modules

_Modules_ let us organise code within a crate for readability and easy reuse.
Modules also allow us to control the _privacy_ of items, because code within a
module is private by default. Private items are internal implementation details
not available for outside use. We can choose to make modules and the items
within them public, which exposes them to allow external code to use and depend
on them.

As an example, let's write a library crate that provides the functionality of a
restaurant. We'll define the signatures of functions but leave their bodies
empty to concentrate on the organisation of the code, rather than the
implementation of a restaurant.

In the restaurant industry, some parts of a restaurant are referred to as
_front of house_ and others as _back of house_. Front of house is where
customers are: this encompasses where the hosts seat customers, servers take
orders and payment, and bartenders make drinks. Back of house is where the
chefs and cooks work in the kitchen, dishwashers clean up, and managers do
administrative work.

To structure our crate in this way, we can organise its functions into nested
modules. Create a new library named `restaurant` by running `cargo new
restaurant --lib`; then enter the following code into _src/lib.rs_ to define
some modules and function signatures. Here's the front of house section:

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

We define a module with the `mod` keyword followed by the name of the module
(in this case, `front_of_house`). The body of the module then goes inside curly
brackets. Inside modules, we can place other modules, as in this case with the
modules `hosting` and `serving`. Modules can also hold definitions for other
items, such as structs, enums, constants, traits, and functions.

By using modules, we can group related definitions together and name why
they're related. Programmers using this code can navigate the code based on the
groups rather than having to read through all the definitions, making it easier
to find the definitions relevant to them. Programmers adding new functionality
to this code would know where to place the code to keep the program organised.

_src/main.rs_ and _src/lib.rs_ are called crate roots; the reason for their
name is that the contents of either of these two files form a module named
`crate` at the root of the crate's module structure, known as the _module
tree_.

Below is the module tree for the `front_of_house` module.

```
crate
`-- front_of_house
    |-- hosting
    |   |-- add_to_waitlist
    |   `-- seat_at_table
    `-- serving
        |-- take_order
        |-- serve_order
        `-- take_payment
```

This tree shows how some of the modules nest inside one another; for example,
`hosting` nests inside `front_of_house`. The tree also shows that some modules
are _siblings_ to each other, meaning they're defined in the same module;
`hosting` and `serving` are siblings defined within `front_of_house`. If module
A is contained inside module B, we say that module A is the _child_ of module B
and that module B is the _parent_ of module A. Notice that the entire module
tree is rooted under the implicit module named `crate`.

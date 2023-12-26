## Table of Contents

  - [Modules](#modules)
  - [Packages and Crates](#packages-and-crates)
  - [Modules Cheat Sheet](#modules-cheat-sheet)
  - [Grouping Related Code in Modules](#grouping-related-code-in-modules)
  - [Paths for Referring to an Item in the Module Tree](#paths-for-referring-to-an-item-in-the-module-tree)
  - [Exposing Paths with the pub Keyword](#exposing-paths-with-the-pub-keyword)
  - [Starting Relative Paths with super](#starting-relative-paths-with-super)
  - [Making Structs and Enums Public](#making-structs-and-enums-public)

## Modules

Rust has a number of features that allow you to manage your code's
organisation, including which details are exposed, which details are private,
and what names are in each scope in your programs. These features, sometimes
collectively referred to as the _module system_, include:

* **Packages**: A Cargo feature that lets you build, test, and share crates
* **Crates**: A tree of modules that produces a library or executable
* **Modules** and **use**: Lets you control the organisation, scope, and
  privacy of paths
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

## Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same
way we use a path when navigating a filesystem. To call a function, we need to
know its path.

A path can take two forms:

* An _absolute path_ is the full path starting from a crate root; for code from
an external crate, the absolute path begins with the crate name, and for code
from the current crate, it starts with the literal `crate`.

* A _relative path_ starts from the current module and uses `self`, `super`, or
    an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

For example if we want to call the `add_to_waitlist` function, we need to find
the path of the `add_to_waitlist` function.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

In the code above, we call the `add_to_waitlist` function using an absolute and
relative path from a new function `eat_at_restaurant` defined in the crate
root. The `eat_at_restaurant` function is part of our library crate's public
API, so we mark it with the `pub` keyword.  These paths are correct, but
there's another problem remaining that will prevent this example from
compiling.

The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path. We then include each of the successive modules until we
make our way to `add_to_waitlist`. You can imagine a filesystem with the same
structure: we'd specify the path `/front_of_house/hosting/add_to_waitlist` to
run the `add_to_waitlist` program; using the `crate` name to start from the
crate root is like using `/` to start from the filesystem root in your shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. **Starting with a module name means
that the path is relative**.

Choosing whether to use a relative or absolute path is a decision you'll make
based on your project, and depends on whether you're more likely to move item
definition code separately from or together with the code that uses the item.
For example, if we move the `front_of_house` module and the `eat_at_restaurant`
function into a module named `customer_experience`, we'd need to update the
absolute path to `add_to_waitlist`, but the relative path would still be valid.
However, if we moved the `eat_at_restaurant` function separately into a module
named `dining`, the absolute path to the `add_to_waitlist` call would stay the
same, but the relative path would need to be updated. **Our preference in
general is to specify absolute paths because it's more likely we'll want to
move code definitions and item calls independently of each other**.

If we try to compile the code above, we'll get error messages that say module
`hosting` is private. We have the correct paths for the `hosting` module and
the `add_to_waitlist` function, but Rust won't let us use them because it does
not have access to the private sections. In Rust, all items (functions,
methods, structs, enums, modules, and constants) are private to parent modules
by default.

Items in a parent module can't use the private items inside child modules, but
items in child modules can use the items in their ancestor modules. This is
because child modules wrap and hide their implementation details, but the child
modules can see the context in which they're defined. Think of the privacy
rules as being like the back office of a restaurant: what goes on in there is
private to restaurant customers, but office managers can see and do everything
in the restaurant they operate.

Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without breaking outer code. However, Rust does give
you the option to expose inner parts of child modules' code to outer ancestor
modules by using the `pub` keyword to make an item public.

## Exposing Paths with the pub Keyword

We want the `eat_at_restaurant` function in the parent module to have access to
the `add_to_waitlist` function in the child module, so we mark the `hosting`
module with the `pub` keyword.

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();
}
```

This will still result in an error despite making `hosting` public. With the
change, we can access `front_of_house` and `hosting` but the _contents_ of
`hosting` are still private; making the module public does not make its
contents public. The `pub` keyword on a module only lets code in its ancestor
module refer to it, not access its inner code. To fix this, we add a `pub`
keyword to make the `add_to_waitlist` function public.

    pub fn add_to_waitlist() {}

In the absolute path, we start with `crate`, the root of our crate's module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn't public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub` and we can access its parent
module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the
first step: rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!

If you plan on sharing your library crate so other projects can use your code,
your public API is your contract with users of your crate that determines how
they can interact with your code.

## Starting Relative Paths with super

We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax. Using
`super` allows us to reference an item that we know is in the parent module,
which can make rearranging the module tree easier when the module is closely
related to the parent, but the parent might be moved elsewhere in the module
tree someday.

The code below models the situation in which a chef fixes an incorrect order
and personally brings it out to the customer. The function
`fix_incorrect_order` defined in the `back_of_house` module calls the function
`deliver_order` defined in the parent module by specifying the path to
`deliver_order` starting with `super`:

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it. We
think the `back_of_house` module and the `deliver_order` function are likely to
stay in the same relationship to each other and get moved together should we
decide to reorganise the crate's module tree. Therefore, we used `super` so
we'll have fewer places to update code in the future if this code gets moved to
a different module.

## Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a
few extra details on the use of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct's fields
will be private. We can make each field public or not on a case-by-case basis.

In the example below, a public `back_of_house::Breakfast` struct is defined
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what's in season and in stock.

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye);
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // the next line won't compile because we're not allowed to see or modify
    // the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In `eat_at_restaurant` we can write and read to the `toast` field using dot
notation because the `toast` field in the `back_of_house::Breakfast` struct is
public.

Note that because `back_of_house::Breakfast` has a private field, the struct
needs to provide a public associated function that constructs an instance of
`Breakfast` (named `summer` in the code above). If `Breakfast` didn't have such
a function, we wouldn't be able to create an instance of `Breakfast` in
`eat_at_restaurant` because we couldn't set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword.

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

Enums aren't very useful unless their variants are public and it would be
annoying to have to annotate all enum variants with `pub` in every case, so the
default for enum variants is to be public. Structs are often useful without
their fields being public, so struct fields follow the general rule of
everything being private by default unless annotated with `pub`.

## Bringing Paths into Scope with the use Keyword

Having to write out the paths to call functions can feel inconvenient and
repetitive. We can create a shortcut to a path with the `use` keyword once, and
then use the shorter name everywhere else in the scope.



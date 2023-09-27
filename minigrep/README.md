## Table of Contents

- [Building a Command Line Program](#building-a-command-line-program)
  - [Accepting Command Line Arguments](#accepting-command-line-arguments)
  - [Reading a File](#reading-a-file)
  - [Refactoring](#refactoring)
    - [Separation of Concerns for Binary Projects](#separation-of-concerns-for-binary-projects)
    - [Extracting the Argument Parser](#extracting-the-argument-parser)
    - [Grouping Configuration Values](#grouping-configuration-values)
    - [Creating a Constructor for Config](#creating-a-constructor-for-config)
    - [Fixing the Error Handling](#fixing-the-error-handling)
    - [Returning a Result Instead of Calling panic!](#returning-a-result-instead-of-calling-panic)
    - [Calling Config::build and Handling Errors](#calling-configbuild-and-handling-errors)
    - [Extracting Logic from main](#extracting-logic-from-main)
    - [Returning Errors from the run Function](#returning-errors-from-the-run-function)
    - [Handling Errors Returned from run in main](#handling-errors-returned-from-run-in-main)
    - [Splitting Code into a Library Crate](#splitting-code-into-a-library-crate)

# Building a Command Line Program

[Building a simplified version of
`grep`](https://doc.rust-lang.org/book/ch12-00-an-io-project.html), which will
take a file path and a string and searches the specified file for the specified
string. This project will combine the following concepts:

* Organising code using modules
* Using vectors and strings, which are collections
* Handling errors
* Using traits and lifetimes
* Writing tests
* Closures (anonymous functions), iterators, and trait objects

## Accepting Command Line Arguments

There are libraries that help with accepting command line arguments, but we can
use the `std::env::args` function provided in Rust's standard library. This
function returns an iterator of the command line arguments passed to a program.
Iterators produce a series of values and we can call the `collect` method on an
iterator to turn it into a collection, such as a vector, that contains all the
elements the iterator produces.

First, we bring the `std::env` module into scope with a `use` statement so we
can use its `args` function. Notice that the `std::env::args` function is
nested in two levels of modules. In cases where the desired function is nested
in more than one module, we have chosen to bring the parent module into scope
rather than the function. By doing so, we can easily use other functions from
`std::env` and it is also less ambiguous than adding `use std::env::args` and
then calling the function with just `args`, because `args` might easily be
mistaken for a function that's defined in the current module.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // the program's name is args[0]
    let query = &args[1];
    let file_path = &args[2];

    // Prints and returns the value of a given expression for quick and dirty
    // debugging.
    dbg!(args);
}
```

On the first line of `main`, we call `env::args`, and we use `collect` to turn
the iterator into a vector containing all the values produced by the iterator.
We can use the `collect` function to create many kinds of collections, so we
explicitly annotate the type of `args` to specify that we want a vector of
strings; `collect` is one function you often need to annotate because Rust is
not able to infer the kind of collection wanted.

## Reading a File

`std:fs` from the standard library handles files.

```rust
use std::env;
use std::fs;

fn main() {
    // --snip--

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
```

`fs::read_to_string` takes the `file_path`, opens the file, and returns a
`std::io::Result<String>` of the file's contents.

Currently the code has a few flaws. The `main` function has multiple
responsibilities and generally functions are clearer and easier to maintain if
each function is responsible for only one idea. The other problem is that we're
not handling errors as well as possible. It is good practice to begin
refactoring early on when developing a program, because it's much easier to
refactor smaller amounts of code.

## Refactoring

Currently, the `main` function performs two tasks: it parses arguments and
reads files. As the program grows, the number of separate tasks the `main`
function handles will increase. As a function gains responsibilities, it
becomes more difficult to reason about, harder to test, and harder to change
without breaking one of its parts. It is best to separate functionality so each
function is responsible for one task.

Another problem is that although `query` and `file_path` are configuration
variables for our program, variables like `contents` are used to perform the
program's logic. The longer `main` becomes, the more variables we need to bring
into scope; the more variables we have in scope, the harder it will be to keep
track of the purpose of each. It is best to group the configuration variables
into one structure to make their purpose clear.

The error handling is only carried out using `expect`, which prints an error
message when reading the file fails. However, reading a file can fail in a
number of ways: for example, the file could be missing or we might not have
permission to open it.

It would be best if all the error-handling core were in one place so future
maintainers only need to look in one place if the error-handling logic needs to
change.

### Separation of Concerns for Binary Projects

The organisational problem of allocating responsibility for multiple tasks to
the main function is common to many binary projects. As a result, the Rust
community has developed guidelines for splitting the separate concerns of a
binary program when `main` starts getting large. This process has the following
steps:

* Split your program into a _main.rs_ and a _lib.rs_ and move your program's
logic to _lib.rs_.

* As long as your command line parsing logic is small, it can remain in
_main.rs_.

* When the command line parsing logic starts getting complicated, extract it
from _main.rs_ and move it to _lib.rs_.

The responsibilities that remain in the `main` function after this process
should be limited to the following:

* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a `run` function in _lib.rs_
* Handling the error if `run` returns an error

This pattern is about separating concerns: _main.rs_ handles running the
program, and _lib.rs_ handles all the logic of the task at hand. Because you
can't test the `main` function directly, this structure lets you test all of
your program's logic by moving it into functions in _lib.rs_. The code that
remains in _main.rs_ will be small enough to verify its correctness by reading
it.

### Extracting the Argument Parser

Create a new function `parse_config` that will contain the functionality for
parsing arguments.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

Instead of assigning the argument value at index 1 to `query` and index 2 to
`file_path` within the `main` function, we pass the whole vector to the
`parse_config` function. The `parse_config` function then holds the logic that
determines which argument goes in which variable and passes the values back to
`main`; `main` no longer has the responsibility of determining how the command
line arguments and variables correspond.

### Grouping Configuration Values

Currently the `parse_config` function returns a tuple that gets broken up into
individual parts. This is a sign that perhaps we don't have the right
abstraction yet.

Another indicator that shows there's room for improvement is the `config` part
of `parse_config`, which implies that the two values we return are related and
are both part of one configuration value. We are not currently conveying this
meaning in the structure of the data other than by grouping two values into a
tuple; we will instead put the two values into one struct and give each of the
struct fields a meaningful name. Doing so will make it easier for future
maintainers to understand how the different values relate to each other and
what their purpose is.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}, config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // --snip--
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

A struct named `Config` is defined to have fields named `query` and
  `file_path`. The signature of `parse_config` now indicates that it returns a
  `Config` value. In the body of `parse_config`, were we used to return string
  slices that reference `String` values in `args`, we now define `Config` to
  contain owned `String` values. The `args` variable in `main` is the owner of
  the argument values and is only letting the `parse_config` function borrow
  them, which means we'd **violate Rust's borrowing rules if `Config` tried to
  take ownership of the values in `args`**.

There are a number of ways we could manage the `String` data; the easiest,
though somewhat inefficient, route is to call the `clone` method on the values.
This will make a full copy of the data for the `Config` instance to own, which
takes more time and memory than storing a reference to the string data.
However, cloning the data also makes our code very straightforward because we
don't have to manage the lifetimes of the references; in this circumstance,
giving up a little performance to gain simplicity is a worthwhile trade-off.

In `main` the `config` variable now contains the instance of `Config` returned
by the `parse_config` function.

### Creating a Constructor for Config

So far, we've extracted the logic responsible for parsing the command line
arguments from `main` and placed it in the `parse_config` function. Doing so
helped us to see that the `query` and `file_path` values were related and that
relationship should be conveyed in our code. We then added a `Config` struct to
name the related purpose to `query` and `file_path` and to be able to return
the values' names as struct field names from the `parse_config` function.

Now that the purpose of the `parse_config` function is to create a `Config`
instance, we can change `parse_config` from a plain function to a function
named `new` that is associated with the `Config` struct. Making this change
will make the code more ioiomatic. We can create instances of types in the
standard library, such as `String`, by calling `String::new`. Similarly, by
changing `parse_config` into a `new` function associated with `Config`, we'll
be able to create instances of `Config` by calling `Config::new`.

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// implement some functionality for a type.
// i.e. associates the new function with Config
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
```

### Fixing the Error Handling

Here's a simple way to check whether enough arguments were provided.

```rust
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```

This is better but `panic!` is more appropriate for a programming problem than
a usage problem.

### Returning a Result Instead of Calling panic!

We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. We're also
going to change the function name from `new` to `build` because many
programmers expect `new` functions to never fail. When `Config::build` is
communicating to `main`, we can use the `Result` type to signal there was a
problem. Then we can change `main` to convert an `Err` variant into a more
practical error for our users without the surrounding text about `thread
'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

```rust
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static stc> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
```

Our `build` function returns a `Result` with a `Config` instance in the success
case and a `&'static str` in the error case. Our error values will always be
string literals that have the `'static` lifetime.

Instead of calling `panic!`, we now return an `Err` value and we've wrapped the
`Config` return value in an `Ok`. These changes make the function conform to
its new type signature.

Returning an `Err` value from `Config::build` allows the `main` function to
handle the `Result` value returned from the `build` function and exit the
process more cleanly in the error case.

### Calling Config::build and Handling Errors

To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::build`.

```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    })

    // --snip--
```

The method `unwrap_or_else`, which is defined on `Result<T, E>` by the standard
library, allows us to define some custom, non-`panic!` error handling. If the
`Result` is an `Ok` value, this method's behaviour is similar to `unwrap`: it
returns the inner value `Ok` is wrapping. However, if the value is an `Err`
value, this method calls the code in the _closure_, which is an anonymous
function we define and pass as an argument to `unwrap_or_else`.
`unwrap_or_else` will pass the inner value of the `Err`, which in this case is
the static string `"not enough arguments"` to our closure in the argument `err`
that appears between the vertical pipes. The code in the closure can then use
the `err` value when it runs.

### Extracting Logic from main

We will extract a function named `run` that will hold all the logic currently
in the `main` function that isn't involved with setting up configuration or
handling errors.

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

// --snip--
```

The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

### Returning Errors from the run Function

Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate the logic around handling errors into `main` in a
user-friendly way.

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

We've made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.

For the error type, we used the _trait object_ `Box<dyn Error>` (and we have
brought `std::error::Error` into scope with a `use` statement at the top).
`Box<dyn Error>` means the function will return a type that implements the
`Error` trait, but we don't have to specify what particular type the return
value will be. This gives us flexibility to return error values that may be of
different types in different error cases.

Second, we have removed the call to `expect` in favour of the `?` operator.
Rather than `panic!` on an error, `?` will return the error value from the
current function for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case. We've
declared the `run` function's success type as `()` in the signature, which
means we need to wrap the unit type value in the `Ok` value. The `Ok(())`
syntax might look a bit strange, but using `()` like this is **the idiomatic
way to indicate that we're calling `run` for its side effects only; it doesn't
return a value we need**.

### Handling Errors Returned from run in main

We'll check for errors and handle them using a technique similar to one we used
with `Config::build` but with a slight difference:

```rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
```

Here we use `if let` rather than `unwrap_or_else` to check whether `run`
returns an `Err` value and call `process::exit(1)` if it does. The `run`
function doesn't return a value that we want to `unwrap` in the same way that
`Config::build` returns the `Config` instance. Because `run` returns `()` in
the success case, we only care about detecting an error, so we don't need
`unwrap_or_else` to return the unwrapped value, which would only be `()`.

### Splitting Code into a Library Crate

Now we will split the _src/main.rs_ file and put some code into the
_src/lib.rs_ file. That way we can test the code and have a _src/main.rs_ file
with fewer responsibilities.

Let's move all the code that isn't the `main` function:

* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::build` function definition

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

Now we need to bring the code we moved to _src/lib.rs_ into the scope of the
binary crate in _src/main.rs_.

```rust
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}
```

We add a `use minigrep::Config` line to bring the `Config` type from the
library crate into the binary crate's scope and we prefix the `run` function
with our crate name.

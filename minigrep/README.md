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
* Closures, iterators, and trait objects

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

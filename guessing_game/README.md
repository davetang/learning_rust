# README

The [_prelude_](https://doc.rust-lang.org/std/prelude/index.html) is the list
of things that Rust automatically imports into every Rust program. It is kept
as small as possible, and is focused on things, particularly traits, which are
used in almost every single Rust program.

If a type you want to use isn't in the prelude, you need to bring that type
into scope explicitly with a `use` statement. Using the `std::io` library
provides a number of useful features, including accepting user input.

Create a new [String](https://doc.rust-lang.org/std/string/struct.String.html):

```rust
let mut guess = String::new();
```

The `::` syntax in the `::new` line indicates that `new` is an associated
function of the `String` type. An _associated function_ is a function that's
implemented on a type. This `new` function creates a new, empty string.

The `stdin` function from the `io` module handles user input:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

The `stdin` function returns an instance of `std::io::Stdin`, which is a type
that represents a handle to the standard input for your terminal. The
`.read_line(&mut guess)` calls the `read_line` method on the standard input
handle to get input. The full job of `read_line` is to take whatever the user
types into standard input and append that into a string (without overwriting
its contents), so we therefore pass that string as an argument. The string
argument needs to be mutable so the method can change the string's content.

The `&` indicates that this argument is a _reference_, which gives you a way to
let multiple parts of your code access one piece of data without needing to
copy that data into memory multiple times.

`read_line` also results a `Result` value; this is an enumeration, often called
an _enum_, which is a type that can be in one of multiple possible states. Each
possible state is called a _variant_. The purpose of `Result` types is to
encode error-handling information.

`Result`'s variants are `Ok` and `Err`. The `Ok` variant indicates the
operation was successful, and inside `Ok` is the successfully generated value.
The `Err` variant means the operation failed, and `Err` contains information
about how or why the operation failed.

Values of the `Result` type, like values of any type, have methods defined on
them. An instance of `Result` has an expect method that can be called. If this
instance of `Result` is an `Err` value, `expect` will cause the program to
crash and display the passed message. If the `read_line` method returns an
`Err`, it would likely be the result of an error coming from the underlying
operating system. If this instance of `Result` is an `Ok` value, `expect` will
take the return value that `Ok` is holding and return just that value to you
for use.

To generate a random number, we can use the `rand` crate.

```rust
use rand::Rng;
let secret_number = rand::thread_rng().gen_range(1..=100);
```

The `Rng` trait defines methods that random number generators implement and
this trait must be in scope for us to use those methods.

# README

Main points:

* Some methods return an enumeration (_enum_), which is a type that can be in
  one of multiple states. Each state is called a _variant_.

* The `read_line` method returns a `Result` type, which is an _enum_,
  containing the variants `Ok` and `Err`. These are used for error-handling.

* A `match` expression is made up of _arms_ that consists of a _pattern_ to
  match against, and the code to execute if the value given to `match` matches
  the pattern. `match` can be used to process `Result` variants.

* Use the `parse` method on strings to convert it to another type.

* Use the `loop` keyword to create a loop; use `break` to exit the loop and
  `continue` to skip.

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

The `rand::thread_rng` function gives us the particular random number generator
we are going to use: one that is local to the current thread of execution and
is seeded by the operating system. Then we call the `gen_range` method on the
random number generator. This method is defined by the `Rng` trait that is
brought into scope with the `use rand::Rng;` statement. The `gen_range` method
takes a range expression as an argument and generates a random number in the
range.

Below is the code that compares the guess with the randomly generated number.

```rust
use std::cmp::Ordering;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // exit loop
                break;
            }
        }
```

`std::cmp::Ordering` is another enum type with the variants `Less`, `Greater`,
and `Equal`.

The `cmp` method compares two values and can be called on anything that can be
compared. It returns a variant of the `Ordering` enum we brought into scope
with the `use` statement. A match expression is used to decide what to do based
on which variant of `Ordering` was returned from the call to `cmp`.

A match expression is made up of _arms_. An arm consists of a _pattern_ to
  match against, and the code that should be run if the value given to `match`
  fits that arm's pattern. Rust takes the value given to `match` and looks
  through each arm's pattern in turn. Patterns and the `match` construct are
  powerful Rust features: they let you express a variety of situations your
  code might encounter and they make sure you handle them all.

Our guess is a `String` type, so we need to convert it to number type to use
`cmp`.

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Rust has a feature called _shadowing_ that is often used to convert a value
from one type to another; it allows us to shadow the previous value of `guess`
with a new one.

The `trim` method on a `String` instance will eliminate any whitespace
(including a newline) at the start and end.

The `parse` method on strings converts a string to another type. The colon in
`let guess: u32` specifies the variable's type; `u32` is an unsigned, 32-bit
integer. The comparison with `secret_number` means that Rust will infer that
`secret_number` should be a `u32` as well, so that the comparison will be
between two values of the same type.

The `parse` method will only work on characters that can logically be converted
into numbers and can easily cause errors. Due to this, the `parse` method also
returns a `Result` type, like the `read_line` method.

The `loop` keyword creates an infinite loop; lines need to be indented inside a
loop. Use a `break` statement to exit the loop.

A refinement to the type conversion step is to ignore non-numbers instead of
  crashing the program.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

A `match` expression is used instead of an `expect` call. Recall that `parse`
  returns a `Result` type, which is an enum that has the variants `Ok` and
  `Err`. The code above uses a `match` expression in the same manner with the
  `Ordering` result of the `cmp` method.

If `parse` is able to successfully convert the string into a number, it will
return an `Ok` value that contains the resultant number. That `Ok` value will
match the first arm's pattern, and the `match` expression will just return the
`num` value that `parse` produced and put inside the `Ok` value.

If `parse` is _not_ able to turn the string into a number, it will return an
`Err` value that contains more information about the error. The `Err` value
does not match the `Ok(num)` pattern in the first `match` arm, but it does
match the `Err(_)` pattern in the second arm. The underscore is a catchall
value; in the code, we want to match all `Err` values, no matter what
information they have inside them. When `Err(_)` matches, the second arm's
code, `continue`, is executed which goes to the next iteration of the `loop`.

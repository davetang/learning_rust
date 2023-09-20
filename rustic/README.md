# README

[Common programming concepts in
Rust](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html).

## Variables

Variables are immutable by default. Use `mut` to make a variable mutable.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6
    println!("The value of x is: {x}");
}
```

_Constants_ are values that are bound to a name and are not allowed to change,
like immutable variables, but there are a few differences between constants and
variables.

* You cannot use `mut` with constants; they are always immutable.
* Constants are declared using `const` instead of `let` and the type of the
  value _must_ be annotated
* Constants may be set only to a constant expression, not the result of a value
  that could only be computed at runtime.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust's naming convention for constants is to use all uppercase with underscores
between words.

When you declare a new variable with the same name as a previous variable using
`let`, the previous variable is _shadowed_ by the second. This means that the
second variable is what the compiler will see when you use the name of the
variable.

Shadowing is different from marking a variable as `mut` because we'll get a
compile-time error if we accidentally try to reassign to the variable without
using the `let` keyword.

## Basic types
## Functions
## Comments
## Control flow



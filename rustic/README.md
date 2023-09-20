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

Use the `fn` keyword to declare new functions. Rust code uses snake_case as the
conventional style for function and variable names, in which all letters are
lowercase and underscores separate words.

The declaration of `another_function` has one parameter named `x`. The type of
`x` is specified as `i32`; you _must_ declare the type of each parameter.

```rust
fn main() {
    another_function();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

Separate multiple parameters with commas.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

Function bodies are made up of a series of statements optionally ending in an
expression. Rust is an expression-based language, so it is important to
distinguish between an expression and a statement.

* **Statements** are instructions that perform some action and do not return a
  value.
* **Expressions** evaluate to a resultant value.

Creating a variable and assigning a value to it with the `let` keyword is a
statement.

Function definitions are also statements. You can't assign a `let` statement to
another variable because statements do not return values.

```rust
// error: expected expression, found statement (`let`)
fn main() {
    let x = (let y = 6);
}
```

The math operation `5 + 6` is an expression that evaluates to the value `11`.
Expressions can be part of statements; the `6` in the statement `let y = 6` is
an expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. A new scope block created with
curly brackets is an expression.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
// The value of y is: 4
```

Note that the `x + 1` line does not have a semicolon at the end. Expressions do
not include ending semicolons. If a semicolon is added to the end of an
expression, it turns into a statement and will not return a value.

Functions can return values to the code that calls them. Return values are not
named but must have their type declared after an arrow (`->`). In Rust, the
return value of the function is synonymous with the value of the final
expression in the block of the body of a function. You can return early from a
function using the `return` keyword and specifying a value, but most functions
return the last expression implicitly.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## Comments
## Control flow



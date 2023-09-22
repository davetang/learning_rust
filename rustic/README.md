## Table of Contents

- [README](#readme)
  - [Variables](#variables)
  - [Basic types](#basic-types)
  - [Vectors](#vectors)
  - [Functions](#functions)
  - [Comments](#comments)
  - [Control flow](#control-flow)

# README

[Common programming concepts explained in a Rust-centric way, i.e.
rustic](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html).

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

Every value in Rust is of a certain _data type_, which tells Rust what kind of
data is being specified so it knows how to work with that data. Rust is a
_statically typed_ language, meaning that it must know the types of all
variables at compile time. The compiler can usually infer the type based on the
value and how the variable is used. In cases, when many types are possible, we
need to add a type annotation.

```rust
let guess: u32 = "1984".parse().expect("Not a number!");
```

A _scalar_ type represents a single value. Rust has four primary scalar types:

1. Integers,
2. Floating-point numbers,
3. Booleans, and
4. Characters.

Integer types in Rust.

| Length  | Signed | Unsigned |
| --      | --     | --       |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

Integer types default to `i32`.

The `isize` and `usize` types depend on the architecture of the computer the
program is running on; 64 bits on a 64-bit architecture and 32 bits on a 32-bit
architecture. The primary situation in which this is used is when indexing some
sort of collection.

Write integer literals as below:

| Number literals  | Example     |
| --               | --          |
| Decimal          | 98_222      |
| Hex              | 0xff        |
| Octal            | 0o77        |
| Binary           | 0b1111_0000 |
| Byte (`u8` only) | b'A'        |

Number literals that can be multiple numeric types allow a type suffix, such as
`57u8`. You can use an underscore (just like in Perl!) to make numbers easier
to read.

**When compiling in release mode with the `--release` flag, Rust does _not_
include checks for integer overflow that cause panics. Instead, if overflow
occurs, Rust performs _two's complement wrapping_. In short, values greater
than the maximum value the type can hold "wrap around" to the minimum of the
values the type can hold. In the case of a `u8`, the value 256 becomes 0, 257
becomes 1, and so on. The program won't panic, but the variable will have a
value that probably isn't what you were expecting. Relying on integer
overflow's wrapping behaviour is considered an error.

Rust has two primitive types for _floating-point numbers_: `f32` and `f64`. The
default type is `f64` because on modern CPUs it is roughly the same speed as
`f32`. All floating-point types are signed. The `f32` type is a
single-precision float and `f64` has double precision.

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Integer division truncates toward zero to the nearest integer.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

The Boolean type in Rust is specified using `bool` and are `true` and `false`;
they are one byte in size.

Rust's `char` type is the language's most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
}
```

`char` literals are specified with single quotes, in contrast to string
literals, which use double quotes. Rust's `char` type is four bytes in size and
represent a Unicode Scalar Value, which means it can represent a lot more than
just ASCII.

_Compound types_ can group multiple values into one type. Rust has two
primitive compound types: tuples and arrays.

A _tuple_ is a general way of grouping together a number of values with a
  variety of types into one compound type. Tuples have a fixed length: once
  declared, they cannot grow or shrink in size.

Below a tuple is created and is bound to the variable `tup`. A pattern is used
with `let` to take `tup` and turn it into three separate variables, `x`, `y`,
and `z`. This is called _destructuring_ because it breaks the single tuple into
three parts.

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

Tuple elements can be directly accessed by using a period (`.`) followed by the
(zero) index of the value, e.g. `let five_hundred = tup.0;`.

The tuple without any values has a special name, _unit_. This value and its
corresponding type are both written `()` and represent an empty value or an
empty return type. Expressions implicitly return the unit value if they don't
return any other value.

An _array_ is another way to have a collection of multiple values. However,
unlike a tuple, every element of an array must have the same type. Arrays in
Rust have a fixed length.

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data allocated on the stack rather than
the heap or when you want to ensure you always have a fixed number of elements.

An array is not as flexible as the vector type; a _vector_ is a similar
collection type provided by the standard library that _is_ allowed to grow or
shrink in size.

You can initialise an array to contain the same value for each element by
specifying the initial value, followed by a semicolon, and then the length of
the array in square brackets.

```rust
let a = [3; 5]
```

An array is a single chunk of memory of a known, fixed size that can be
allocated on the stack. Use indexing to access elements of an array.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
```

## Vectors

[Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) (`Vec<T>`) are a
collection type and can store more than one value in a single data structure
that puts all the values next to each other in memory. Vectors can only store
values of the same type. They are useful when you have a list of items, such as
the lines of text in a file or the prices of items in a shopping cart.

Use `Vec::new` to create a new empty vector; a type annotation is included
because the vector is empty and Rust does not know what kind of elements will
be stored.

```rust
let v: Vec<i32> = Vec::new();
```

Rust conveniently provides the `vec!` macro, which will create a new vector
that holds the values you provide it. Rust infers that the type of `v` is
`Vec<i32>` because it's the default integer type.

```rust
let v = vec![1, 2, 3];
```

Use the `push` method to add elements to a vector and the `pop` method to
remove and return the last element.

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);

// removes 8
v.pop();
```

There are two ways to reference a value stored in a vector: via indexing or
using the `get` method. An `Option<&T>` is returned when the `get` method is
used with the index and this can be used with `match`.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

There are two ways to reference an element because you can choose how your
program behaves when you use an index value outside the range of existing
elements.

```rust
let v = vec![1, 2, 3, 4, 5];

// panic
let does_not_exist = &v[100];

// returns None without panicking
let does_not_exist = v.get(100);
```

When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules to ensure that the reference and any other
references to the contents of the vector remain valid.

Note that the following code will result in a compile error.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
}
```

This error is due to the way vectors work: because vectors put the values next
to each other in memory, adding a new element onto the end of the vector might
require allocating new memory and copying the old elements to the new space, if
there isn't enough room to put all the elements next to each other where the
vector is currently stored. In that case, the reference to the first element
would be pointing to deallocated memory. The borrowing rules prevent programs
from ending up in that situation.

Use a `for` loop to get immutable references to each element in a vector.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}
```

We can also iterate over mutable references to each element in order to make
changes to all the elements. To change the value that the mutable reference
refers to, we have to use the `*` dereference operator to get to the value in
`i` before we can use the `+=` operator.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Vectors can only store values that are the same type but we can use an Enum to
store multiple types. The variants of an enum are defined under the same enum
type, so when we need one type to represent elements of different types, we can
define and use an enum.

For example, say we wanted to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types and all the enum variants will be considered the same type. Then we
can create a vector to hold that enum and so, ultimately, hold different types.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. We
must also be explicit about what types are allowed in a vector. If Rust allowed
a vector to hold any type, there would be a chance that one or more of the
types would cause errors with the operations performed on the elements of the
vector. Using an enum plus a `match` expression means that Rust will ensure at
compile time that every possible case is handled.

If you don't know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won't work. Instead, you can use a trait
object.

A vector is freed when it goes out of scope, like any other `struct`. When the
  vector get dropped, all of its contents are also dropped, meaning the
  integers it holds will be cleaned up. The borrow checker ensures that any
  references to contents of a vector are only used while the vector itself is
  valid.

```rust
{
    let v = vec![1, 2, 3, 4];
} // <- v goes out of scope and is freed here
```

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

```rust
//
```

## Control flow

The most common constructs that let you control the flow of execution of Rust
code are `if` expressions and loops.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("{number} is divisible by 4");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("{number} is divisible by 2");
    } else {
        println!("{number} is not divisible by 4, 3, or 2");
    }
}
// 6 is divisible by 3
```

Blocks of code associated with the conditions in `if` expressions are sometimes
called _arms_, just like the arms in `match` expressions.

Conditions in `if` expressions _must_ be a `bool`. Rust will not automatically
try to convert non-Boolean types to a Boolean.

If there are too many `else if` expressions, consider using `match`.

Since `if` is an expression, we can use it on the right side of a `let`
statement to assign the outcome to a variable. The `if` arm and the `else` arm
need to be the same type or else there will be a compile error.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

Rust has three kinds of loops:

1. `loop`,
2. `while`, and
3. `for`.

The `loop` keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop using the `break` keyword. The
`continue` keyword skips over any remaining code in the current iteration of
the loop and will continue in the next iteration.

One of the uses of a `loop` is to retry an operation that might fail, such as
checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. You can
add the value to be returned after the `break` expression.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

For loops within loops, `break` and `continue` apply to the innermost loop at
that point. A _loop label_ can be used on a loop, which can then be used with
`break` or `continue`. Loop labels must begin with a single quote.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

Below is an example of a `while` loop.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

You can use the `while` construct to loop over the elements of a collection,
such as an array.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

However, the above approach is error prone. For example, if the array is
changed but not the condition. It is also slow because the compiler adds
runtime code to perform the conditional check of whether the index is within
the bounds of the array on every iteration through the loop.

A more concise alternative is a `for` loop.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

Use `rev` to reverse a range.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

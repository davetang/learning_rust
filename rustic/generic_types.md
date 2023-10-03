## Table of Contents

  - [Generic Data Types](#generic-data-types)
  - [Functions](#functions)
  - [Structs](#structs)
  - [Enums](#enums)
  - [Methods](#methods)
  - [Performance](#performance)

## Generic Data Types

We use generics to create definitions for items like function signatures or
structs, which we can then use with many different concrete data types.

## Functions

When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code
duplication.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}
```

To parameterise the types in a new single function, we need to name the type
parameter, just as we do for the value parameters to a function. You can use
any identifier as a type parameter name. But we'll use `T` because, by
convention, type parameter names in Rust are short, often just a letter, and
Rust's type-naming convention is UpperCamelCase. Short for "type,", `T` is the
default choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the
parameter name in the signature so the compiler knows what the name means.
Similarly, when we use a type parameter name in a function signature, we have
to declare the type parameter name before we use it. To define the generic
`largest` function, place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list.

```rust
fn largest<T>(list: &[T]) -> &T {}
```

We read this definition as: the function largest is generic over some type `T`.
This function has one parameter named `list`, which is a slice of values of
type `T`. The `largest` function will return a reference to a value of the same
type `T`.

## Structs

We can also define structs that use a generic type parameter in one or more
fields using the `<>` syntax.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

You can use as many generic type parameters in a definition as you want, but
using more than a few makes your code hard to read. If you are finding you need
lots of generic types in your code, it could indicate that your code needs
restructuring into smaller pieces.

## Enums

We can define enums to hold generic data types in their variants.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The `Option<T>` enum is generic over type `T` and has two variants: `Some`,
which holds one value of type `T`, and a `None` variant that does not hold any
value. By using the `Option<T>` enum, we can express the abstract concept of an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.

Enums can use multiple generic types as well.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`).

## Methods

We can implement methods on structs and enums and use generic types in their
definitions too.

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

The code above defined a method named `x` on `Point<T>` that returns a
reference to the data in the field `x`.

Note that we have to declare `T` just after `impl` so we can use `T` to specify
that we're implementing methods on the type `Point<T>`. By declaring `T` as a
generic type after `impl`, Rust can identify that the type in the angle
brackets in `Point` is a generic type rather than a concrete type. Methods
written within an `impl` that declares the generic type will be defined on any
instance of the type, no matter what concrete type ends up substituting for the
generic type.

We can also specify constraints on generic types when defining methods on the
type. We could implement methods only on `Point<f32>` instances rather than on
`Point<T>` instances with any generic type.

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Generic type parameters in a struct definition aren't always the same as those
you use in that same struct's method signatures. The code below uses the
generic types `X1` and `Y1` for the `Point` struct and `X2` `Y2` for the
`mixup` method signature to make the example clearer. The method creates a new
`Point` instance with the `x` value from the `self Point` (of type `X1`) and
the `y` value from the passed-in `Point` (of type `Y2`).

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## Performance

The use of generic types won't make your program run any slower than it would
with concrete types.

Rust accomplishes this by performing monomorphisation of the code using
generics at compile time. _Monomorphisation_ is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled. The process of monomorphisation makes Rust's generics extremely
efficient at runtime.

## Table of Contents

  - [Structure](#structure)
  - [Defining and Instantiating Structs](#defining-and-instantiating-structs)
  - [Using the Field Init Shorthand](#using-the-field-init-shorthand)
  - [Creating Instances from Other Instances with Struct Update Syntax](#creating-instances-from-other-instances-with-struct-update-syntax)
  - [Using Tuple Structs Without Named Fields to create Different Types](#using-tuple-structs-without-named-fields-to-create-different-types)
  - [Unit-Like Structs Without Any Fields](#unit-like-structs-without-any-fields)
  - [Ownership of Struct Data](#ownership-of-struct-data)

## Structure

A _struct_, or _structure_, is a custom data type that lets you package
  together and name multiple related values that make up a meaningful group. If
  you're familiar with an object-oriented language, a _struct_ is like an
  object's data attributes.

## Defining and Instantiating Structs

Structs are similar to tuples in that both hold multiple related values. Like
tuples, the pieces of a struct can be different types. Unlike with tuples, in a
struct you'll name each piece of data so it's clear what the values mean.
Adding these names means that structs are more flexible than tuples; you don't
have to rely on the order of the data to specify or access the values of an
instance.

Use the keyword `struct` to define a struct and name the entire struct. A
struct's name should describle the significance of the pieces of data being
grouped together. Then, inside curly brackets, define the names and types of
the pieces of data, which are called _fields_.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To use a struct after we've defined it, we create an _instance_ of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing
_key:value_ pairs, where the keys are the names of the fields and the values
are the data we want to store in those fields.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("dtang"),
        email: String::from("me@davetang.org"),
        sign_in_count: 1,
    };
}
```

To get a specific value from a struct, we use got notation; to access the email
address use `user1.email`. If the instance is mutable, we can change a value by
using the dot notation and assigning into a particular field.

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("dtang"),
        email: String::from("me@davetang.org"),
        sign_in_count: 1,
    };
    user1.email = String::from("davetingpongtang@gmail.com");
}
```

Note that the entire instance must be mutable; Rust does not allow us to mark
only certain fields as mutable. As with any expression, we can construct a new
instance of the struct as the last expression in the function body to
implicitly return that new instance.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

## Using the Field Init Shorthand

We can use the _field init shorthand_ syntax to rewrite `build_user` so it
behaves exactly the same but does not have the repetition of `username` and
`email`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

The function above creates a new instance of the `User` struct. Because the
`email` field and the email parameter have the same name, we only need to write
`email` rather than `email: email`.

## Creating Instances from Other Instances with Struct Update Syntax

It is often useful to create a new instance of a struct that includes most of
the values from another instance, but changes some. This can be done using
_struct update syntax_.

The following does not use the update syntax.

```rust
fn main() {
    // --snip --

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
```

Use the syntax `..` to specify that the remaining fields should have the same
value as the given instance. The instance must come last to specify that any
remaining fields should get their values from the corresponding fields in the
other instance.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

Note that the struct update syntax users `=` like an assignment; this is
because it moves the data. In the above example, we can no longer use `user1`
as a whole after creating `user2` because the `String` in the `username` field
of `user1` was moved into `user2`. If we had given `user2` new `String` values
for bother `email` and `username`, and thus only used the `active` and
  `sign_in_count` values from `user1`, then `user1` would still be valid after
  creating `user2`. This is because both `active` and `sign_in_count` are types
  that implement the `Copy` trait.

## Using Tuple Structs Without Named Fields to create Different Types

Rust also supports structs that look similar to tuples, called _tuple structs_.
Tuple structs have the added meaning the struct name provides but don't have
names associated with their fields; rather, they just have the types of the
fields. Tuple structs are useful when you want to give the whole tuple a name
and make the tuple a different type from other tuples, and when naming each
field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name
followed by the types in the tuple.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

The `black` and `origin` values are different types because they are instances
of different tuple structs. Each struct you define is its own type, even though
the fields within the struct might have the same types. For example, a function
that takes a parameter of type `Color` cannot take a `Point` as an argument,
even though both types are made up of three `i32` values. Otherwise, tuple
struct instances are similar to tuples in that you can destructure them into
their individual pieces, and you can use a `.` followed by the index to access
an individual value.

## Unit-Like Structs Without Any Fields

You can also define structs that don't have any fields; these are called
`unit-like structs` because they behave similarly to `()`, the unit type.
Unit-like structs can be useful when you need to implement a trait on some type
but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

A unit-like struct does not need curly brackets; just use the `struct` keyword.

## Ownership of Struct Data

In the `User` struct definition example, the owned `String` type was used
rather than the `&str` string slice type. This was deliberately done because we
want each instance of this struct to own all of its data and for that data to
be valid for as long as the entire struct is valid.

It is also possible for structs to store references to data owned by something
else, but to do so requires the use of `lifetimes`. Lifetimes ensure that the
  data referenced by a struct is valid for as long as the struct is. The
  following code, which does not specify lifetimes, does not work and the
  compiler will specify that it needs lifetime specifiers.

```rust
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "user1",
        email: "user1@example.com",
        sign_in_count: 1,
    };
}
```

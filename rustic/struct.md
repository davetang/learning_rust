## Table of Contents

  - [Structure](#structure)
  - [Defining and Instantiating Structs](#defining-and-instantiating-structs)

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

## Table of Contents

  - [Enumerations](#enumerations)
  - [The Option Enum and Its Advantages Over Null Values](#the-option-enum-and-its-advantages-over-null-values)
  - [The match Control Flow Construct](#the-match-control-flow-construct)
  - [Patterns That Bind to Values](#patterns-that-bind-to-values)
  - [Matching with Option<T>](#matching-with-optiont)
  - [Matches Are Exhaustive](#matches-are-exhaustive)
  - [Catch-all Patterns and the _ Placeholder](#catch-all-patterns-and-the-_-placeholder)
  - [Concise Control Flow with if let](#concise-control-flow-with-if-let)

## Enumerations

Enumerations, also referred to as _enums_, allow you to define a type by
enumerating its possible _variants_. Where structs give you a way of grouping
together related fields and data, like a `Rectangle` with its `width` and
`height`, enums give you a way of saying **a value is one of a possible set of
values**. For example, we may want to say that `Rectange` is one of a set of
possible shapes that also includes `Circle` and `Triangle`. Rust allows us to
encode these possibilities as an enum.

Currently, there are two major standard used for IP addresses: version four and
version six. Since these are the only possibilities for an IP address, we can
_enumerate_ all possible variants, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address, but not
both at the same time. This property of IP addresses makes the enum data
structure appropriate because an enum value can only be one of its variants.
Both version four and version six addresses are still fundamentally IP
addresses, so they should be treated as the same type when the code is handling
situations that apply to any kind of IP address.

We can express this concept in code by defining an `IpAddrKind` enumeration and
listing the possible kinds an IP address can be, `V4` and `V6`.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

We can create instances of each of the two variants of `IpAddrKind` with the
following.

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier; this is
useful because now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the
same type; `IpAddrKind`. We can then define a function that takes any
`IpAddrKind`.

```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Using enums has even more advantages. At the moment our IP address type has no
way to store IP address _data_; we only know the _kind_. We _can_ use structs
as follows.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

However, we can do the same using just an emum; the name of each enum variant
that is defined also becomes a function that constructs an instance of the
enum. `IpAddr::V4()` is a function call that takes a `String` argument and
returns an instance of the `IpAddr` type. We automatically get this constructor
function defined as a result of defining the enum.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

There's another advantage to using an enum rather than a struct: each variant
can have different types and amounts of associated data. Version four IP
addresses will always have four numeric components that will have values
between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but
still express `V6` addresses as one `String` value, we wouldn't be able to with
a struct.

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

The standard library already has a definition for IP addresses. The standard
library defines `IpAddr` with the exact enum and variants as we have defined
but it embeds the address data inside the variants in the form of two different
structs.

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

The code illustrates that you can put any kind of data inside an enum variant:
strings, numeric types, or structs; you can even include another enum. Also,
standard library types are often not much more complicated than what you might
come up with.

Here's another enum example with a wide variety of types embedded in its
variants.

* `Quit` has no data associated with it
* `Move` has named fields, like a struct
* `Write` includes a single `String`
* `ChangeColor` includes three `i32` values

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Defining an enum with variants of different types is similar to defining
different kinds of struct definitions, except the enum does not use the
`struct` keyword and all the variants are grouped together.

There is one more similarity between enums and structs: just as we are able to
define methods on structs using `impl`, we are also able to define methods on
enums. Here's a method named `call` that we could define on the `Message` enum.

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The body of the method would use `self` to get the value that we called the
method on.

## The Option Enum and Its Advantages Over Null Values

The `Option` type is an enum defined by the standard library and encodes the
very common scenario in which a value could be something or it could be
nothing. For example, if you request the first item in a non-empty list, you
would get a value. If you request the first item in an empty list, you would
get nothing. Expressing this concept in terms of the type system means the
compiler can check whether you have handled all the cases you should be
handling; this functionality can prevent bugs that are extremely common in
other programming languages.

Programming language design if often thought of in terms of which features you
include, but the features you exclude are important too. Rust does not have the
null feature that many other languages have. _Null_ is a value that means there
is no value there. In languages with null, variables can always be in one of
two states: null or not-null.

In his 2009 presentation "Null References: The Billion Dollar Mistake," Tony
Hoare, the inventor of null, has this to say:

>I call it my billion-dollar mistake. At that time, I was designing the first
   comprehensive type system for references in an object-oriented language. My
   goal was to ensure that all use of references should be absolutely safe,
   with checking performed automatically by the compiler. But I couldn't resist
   the temptation to put in a null reference, simply because it was so easy to
   implement. This has led to innumerable errors, vulnerabilities, and system
   crashes, which have probably caused a billion dollars of pain and damage in
   the last forty years.

The problem with null values is that if you try to use a null value as a
not-null value, you'll get an error of some kind. Because this null or not-null
property is pervasive, it's extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a
null is a value that is currently invalid or absent for some reason.

The problem is not really with the concept but with the particular
implementation. As such, Rust does not have nulls, but it does have an enum
that can encode the concept of a value being present or absent. This enum is
`Option<T>`, and it is defined by the standard library as follows:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

The `<T>` syntax is a generic type parameter and it means that the `Some`
variant of the `Option` enum can hold one piece of data of any type and that
each concrete type that gets used in place of `T` makes the overall `Option<T>`
type a different type.

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

Rust can infer the types of `some_number` and `some_char` but for
`absent_number`, Rust requires us to annotate the overall `Option` type: the
compiler can't infer the type that the corresponding `Some` variant will hold
by looking only at a `None` value.

When we have a `Some` value, we know that a value is present and the value is
held within the `Some` type. When we have a `None` value, in some sense it
means the same thing as null: we do not have a valid value. So why is having
`Option<T>` any better than having null?

In short, because `Option<T>` and `T` (where `T` can be any type) are different
types, the compiler won't let us use an `Option<T>` value as if it were
definitely a valid value. For example, the following code won't compile because
it is trying to add an `i8` to an `Option<i8>`:

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

This is because `i8` and `Option<i8>` are different types and Rust does not
understand how they are added.

We use `Option<T>` when a value can be null and we can only use its value when
we convert an `Option<T>` to a `T`. This conversion requires us to explicitly
handle the case when the value is null. Generally, this helps catch one of the
most common issues with null: assuming that something is not null when it
actually is.

## The match Control Flow Construct

The control flow construct called `match` allows you to compare a value against
a series of patterns and then execute code based on which pattern matches.
Patterns can be made up of literal values, variable names, wildcards, and many
other things. The power of `match` comes from the expressiveness of the
patterns and the fact that the compiler confirms that all possible cases are
handled.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Note that `match` seems very similar to a conditional expression used with `if`
but there's a big difference: with `if`, the condition needs to evaluate to a
Boolean value, but with `match` it can be any type.

Use curly brackets if the arm code is long.

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

## Patterns That Bind to Values

Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.

```rust
#[derive(Debug)] // for inspecting the state
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska))
```

In the example above, the binding for `state` will be the value
`UsState::Alaska`, which gets printed out.

## Matching with Option<T>

Let's stay we want to write a function that takes an `Option<i32>` and, it
there's a value inside, adds 1 to that value. If there isn't a value inside,
the function should return the `None` value and not attempt to perform any
operations.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Combining `match` and enums is useful in many situations. You will see this
pattern a lot in Rust code:

1. `match` against an enum
2. Bind a variable to the data inside
3. Execute code based on the binding

## Matches Are Exhaustive

The arms' patterns of `match` must cover all possibilities. The following won't
compile because the `None` case is not handled.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

Matches in Rust are _exhaustive_: we must exhaust every last possibility in
order for the code to be valid.

## Catch-all Patterns and the _ Placeholder

Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we are implementing a
  game where, if you roll a three, your player doesn't mover but instead gets a
  new fancy hat. If you roll a seven, your player loses a fancy hat. For all
  other values, your player moves that number of spaces on the game board.

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

Rust also has a pattern we can use when we want a catch-all but don't want to
_use_ the value in the catch-all pattern: `_` is a special pattern that matches
any value and does not bind to that value. This tells Rust we aren't going to
use the value, so Rust won't warn us about an unused variable.

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

Use the unit value, i.e. the empty tuple type, when we don't want to run any
code.

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}
```

## Concise Control Flow with if let

The `if let` syntax lets you combine `if` and `let` into a less verbose way to
handle values that match one pattern while ignoring the rest. Consider the
following:

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

Since we are using `match` we need to add `_ => ()`. We could write the above
code using `if let`.

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

The syntax `if let` takes a pattern and an expression separated by an equal
sign. It works the same way as a `match`, where the expression is given to the
`match` and the pattern is its first arm. The code in the `if let` block isn't
run if the value doesn't match the pattern.

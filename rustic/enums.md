## Table of Contents

  - [Enumerations](#enumerations)

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

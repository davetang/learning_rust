## Table of Contents

  - [Strings](#strings)
  - [What Is a String?](#what-is-a-string)
  - [Creating a New String](#creating-a-new-string)
  - [Updating a String](#updating-a-string)
  - [String concatenation](#string-concatenation)
  - [Indexing into Strings](#indexing-into-strings)
  - [Bytes and Scalar Values and Grapheme Clusters](#bytes-and-scalar-values-and-grapheme-clusters)
  - [Slicing Strings](#slicing-strings)
  - [Strings Are Not So Simple](#strings-are-not-so-simple)

## Strings

Programmers new to Rust commonly get stuck on strings for a combination of
three reasons:

1. Rust's propensity for exposing possible errors,
2. Strings being a more complicated data structure than many programmers give
   them credit for, and
3. UTF-8.

These factors combine in a way that can seem difficult when you're coming from
other programming languages.

Strings are discussed in the context of collections because **strings are
implemented as a collection of bytes**, plus some methods to provide useful
functionality when those bytes are interpreted as text.

These notes contain the operations on `String` that every collection type has,
such as creating, updating, and reading. It will also discuss the ways in which
`String` is different from the other collections, namely how indexing into a
`String` is complicated by the differences between how people and computers
interpret `String` data.

## What Is a String?

Rust has only one string type in the **core language**, which is the string
slice `str` that is usually seen in its borrowed form `&str`. `String slices`
are references to some UTF-8 encoded string data stored elsewhere. String
literals are stored in the program's binary and are therefore string slices.

The `String` type, which is provided by Rust's **standard library** rather than
coded into the core language, is a growable, mutable, owned, UTF-8 encoded
string type. When Rust programmers refer to "strings" in Rust, they might be
referring to either the `String` or the string slice `&str` types, not just one
of those types. Both types are heavily used in Rust's standard library and both
`String` and string slices are UTF-8 encoded.

## Creating a New String

Many of the same operations available with `Vec<T>` are available with `String`
as well, because `String` is actually implemented as a wrapper around a vector
of bytes with some extra guarantees, restrictions, and capabilities. An example
of a function that works the same way with `Vec<T>` and `String` is the new
function to create an instance.

The following code creates a new empty string called `s`, which we can then
load data into.

```rust
let mut s = String::new();
```

We use the `to_string` method when we have some initial data to start the
string with; this method is available on any type that implements the `Display`
trait, as string literals do.

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works directly on a string literal
let s = "initial contents".to_string();
```

The function `String::from` can also be used to create a `String` from a string
literal.

```rust
let s = String::from("initial contents");
```

Since strings are UTF-8 encoded, we can include any properly encoded data in
them.

## Updating a String

A `String` can grow in size and its contents can change, just like the contents
  of a `Vec<T>`, if you push more data into it. In addition, you can
  conveniently use the `+` operator or the `format!` macro to concatenate
  `String` values.

We can grow a `String` by using the `push_str` method to append a string slice.

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don't necessarily want to take ownership of the
parameter. The code below works because `push_str` does not take ownership and
`s2` is still valid.

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

The `push` method takes a _single_ character as a parameter and adds it to the
`String`.

```rust
let mut s = String::from("lo");
s.push('l');
```

`s` will contain `lol`.

## String concatenation

One way to combine two existing strings is to use the `+` operator.

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved and is no longer valid
```

The string `s3` will contain `Hello, world!`. The reason `s1` is no longer
valid after the addition, and the reason we used a reference to `s2`, has to do
with the signature of the method that's called when we use the `+` operator.
The `+` operator uses the `add` method, whose signature looks something like
this:

```rust
fn add(self, s: &str) -> String {
```

In the standard library, you'll see `add` defined using generics and associated
types. Here, we've substituted in concrete types, which is what happens when we
call this method with `String` values. This signature gives us the clues we
need to understand the tricky bits of the `+` operator.

First, `s2` has an `&`, meaning that we're adding a _reference_ of the second
string to the first string. This is because of the `s` parameter in the `add`
function: we can only add a `&str` to a `String`; we can't add two `String`
values together. However, the type of `&s2` is `&String` and not `&str`, as
specified in the second parameter to `add`.

The reason we're able to use `&s2` in the call to `add` is that the compiler
can _coerce_ the `&String` argument into a `&str`. When we call the `add`
method, Rust uses a `deref coercion`, which here turns `&2` into `&2[..]`.
Because `add` does not take ownership of the `s` parameter, `s2` will still be
a valid `String` after this operation.

Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does _not_ have an `&`. This means `s1` will be moved into the
`add` call and will no longer be valid after that. So although `let s3 = s1 +
&s2;` looks like it will copy both strings and create a new one, this statement
actually takes ownership of `s1`, appends a copy of the contents of `s2`, and
then returns ownership of the result.

If we need to concatenate multiple strings, the behaviour of the `+` operator
gets a bit messy.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "_" + &s3;
```

`s` will be `tic-tac-toe` but it looks messy. For more complicated string
combining, use the `format!` macro.

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");
```

The `format!` macro works like `println!` but returns a `String` with the
contents instead. The code generated by the `format!` macro uses references so
that this call does not take ownership of any of its parameters.

## Indexing into Strings

In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if you try to access parts of a `String` using indexing syntax in Rust, you'll
  get an error saying that Rust strings do not support indexing due to the way
  Rust stores strings in memory.

```rust
let s1 = String::from("hello");
let h = s1[0];
```

A `String` is a wrapper over a `Vec<u8>`. Consider the following string:

```rust
let hello = String::from("Hola");
```

`hello`'s `len` will be 4, which means the vector storing the string "Hola" is
4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8.

Now consider this string:

```rust
//                        012345678901
let hello = String::from("Здравствуйте");
```

This string is not 12 bytes long but 24 because that's the number of bytes it
takes to encode it in UTF-8 since each Unicode scalar value in that string
takes 2 bytes of storage. Therefore, an index into the string's bytes will not
always correlate to a valid Unicode scalar value.

## Bytes and Scalar Values and Grapheme Clusters

There are three relevant ways to look at strings from Rust's perspective: as

1. Bytes,
2. Scalar values, and
3. Grapheme clusters (the closest thing to what we would call _letters_)

If we look at the Hindi word "नमस्ते" written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

That's 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust's `char` type is, those
bytes are:

```
['न', 'म', 'स', '्', 'त', 'े']
```

There are six `char` values here, but the fourth and sixth are not letters;
they are diacritics that don't make sense on their own. Finally, if we look at
them as grapheme clusters, we'd get what a person would call the four letters
that make up the Hindi word:

```
["न", "म", "स्", "ते"]
```

Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs.

A final reason Rust does not allow us to index into a `String` to get a
  character is that indexing operations are expected to always take constant
  time (O(1)). But it isn't possible to guarantee that performance with a
  `String`, because Rust would have to walk through the contents from the
  beginning to the index to determine how many valid characters there were.

## Slicing Strings

Indexing into a string is often a bad idea because it's not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. If you really need to use
indices to create string slices, Rust asks you to be more specific.

Rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:

```rust
let hello = "Здравствуйте";
let s = &hello[0..4];
```

`s` will be a `&str` that contains the first 4 bytes of the string. If we were
to try to slice only part of a character's bytes with something like
`&hello[0..1]`, Rust would panic at runtime in the same way as if an invalid
index were accessed in a vector.

**You should use ranges to create string slices with caution, because doing so
can crash your program**.

The best way to operate on pieces of strings is to be explicit about whether
you want characters or bytes. For individual Unicode scalar values, use the
`chars` method. Use the `bytes` method to return each raw byte but be sure to
remember that valid Unicode scalar values may be made up of more than 1 byte.

## Strings Are Not So Simple

To summarise, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behaviour
for all Rust programs, which means programmers have to put more thought into
  handling UTF-8 data upfront. This trade-off exposes more of the complexity of
  strings than is apparent in other programming languages, but it prevents you
  from having to handle errors involving non-ASCII characters later in your
  development life cycle.

The good news is that the standard library offers a lot of functionality built
off the `String` and `&str` types to help handle these complex situations
correctly. Be sure to check out the documentation for useful methods like
`contains` for searching in a string and `replace` for substituting parts of a
string with another string.

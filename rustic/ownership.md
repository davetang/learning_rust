## Table of Contents

- [Understanding Ownership](#understanding-ownership)
  - [The Stack and the Heap](#the-stack-and-the-heap)
  - [Ownership Rules](#ownership-rules)
  - [Memory and Allocation](#memory-and-allocation)
  - [Variables and Data Interacting with Move](#variables-and-data-interacting-with-move)
  - [Variables and Data Interacting with Clone](#variables-and-data-interacting-with-clone)
  - [Stack-Only Data: Copy](#stack-only-data-copy)
  - [Ownership and Functions](#ownership-and-functions)
  - [Return Values and Scope](#return-values-and-scope)

# Understanding Ownership

[Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
is Rust's most unique feature and has deep implications for the rest of the
language. It enables Rust to make memory safety guarantees without needing a
garbage collector, so it's important to understand how ownership works. Related
features include: borrowing, slices, and how Rust lays data out in memory.

Ownership is a set of rules that govern how a Rust program manages memory. All
programs have to manage the way they use a computer's memory while running.
Some languages have garbage collection that regularly looks for no-loner-used
memory as the program runs; in other languages, the programmer must explicitly
allocate and free the memory. Rust uses a third approach: **memory is managed
through a system of ownership with a set of rules that the compiler checks**.
If any of the rules are violated, the program won't compile. None of the
features of ownership will slow down your program while it is running.

## The Stack and the Heap

Many programming languages don't require you to think about the stack and the
heap very often. But in a systems programming language like Rust, whether a
value is on the stack or the heap affects how the language behaves and why you
have to make certain decisions.

Both the stack and the heap are parts of memory available to your code to use
at runtime, but they are structured in different ways. The stack stores values
in the order it gets them and removes the values in the opposite order. This is
referred to as _last in, first out_. Think of a stack of plates: when you add
more plates, you put them on top of the pile, and when you need a plate, you
take one off the top. Adding or removing plates from the middle or bottom
wouldn't work as well. Adding data is called _pushing onto the stack_, and
removing data is called _popping off the stack_. All data stored on the stack
must have a known, fixed size. Data with an unknown size at compile time or a
size that might change must be stored on the heap instead.

The heap is less organised: when you put data on the heap, you request a
certain amount of space. The memory allocator finds an empty spot in the heap
that is big enough, marks it as being in use, and returns a _pointer_, which is
the address of that location. This process is called _allocating on the heap_
and is sometimes abbreviated as just _allocating_ (pushing values onto the
stack is not considered allocating). Because the pointer to the heap is a
known, fixed size, you can store the pointer on the stack, but when you want
the actual data, you must follow the pointer. Think of being seated at a
restaurant. When you enter, you state the number of people in your group, and
the host finds an empty table that fits everyone and leads you there. If
someone in your group comes late, they can ask where you've been seated to find
you.

Pushing to the stack is faster than allocating on the heap because the
allocator never has to search for a place to store new data; that location is
always at the top of the stack. Comparatively, allocating space on the heap
requires more work because the allocator must first find a big enough space to
hold the data and then perform bookkeeping to prepare for the next allocation.

Accessing data in the heap is slower than accessing data on the stack because
you have to follow a pointer to get there. Contemporary processors are faster
if they jump around less in memory. Continuing the analogy, consider a server
at a restaurant taking orders from many tables. It's most efficient to get
all the orders at one table before moving on to the next table. Taking an
order from table A, then an order from table B, then one from A again would
be a much slower process. By the same token, a processor can do its job
better if it works on data that's close to other data (as it is on the stack)
rather than farther away (as it can be on the heap).

When your code calls a function, the values passed into the function
(including, potentially, pointers to data on the heap) and the function's local
variables get pushed onto the stack. When the function is over, those values
get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimising
the amount of duplicate data on the heap, and cleaning up unused data on the
heap so you don't run out of space are all problems that ownership addresses.
Once you understand ownership, you won't need to think about the stack and the
heap very often, but knowing that the main purpose of ownership is to manage
heap data can help explain why it works the way it does.

## Ownership Rules

* Each value in Rust has an _owner_.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

As a first example of ownership, we'll look at the _scope_ of some variables. A
scope is the range within a program for which an item is valid.

```rust
{ // s is not valid here as it is not declared yet
    let s = "hello"; // s is valid from this point forward

} // this scope is now over, and s is no longer valid
```

* When `s` comes into scope, it is valid.
* It remains valid until it goes _out of_ scope.

Types that are of a known size can be stored on the stack and popped off the
stack when their scope is over, and can be quickly and trivially copied to make
a new, independent instance if another part of code needs to use the same value
in a different scope. To illustrate the rules of ownership, we need to look at
data that is stored on the heap and explore how Rust knows when to clean up
that data. The `String` type serves as a great example.

The String type manages data allocated on the heap and as such is able to store
an amount of text that is unknown to us at compile time. You can create a
`String` from a string literal using the `from` function:

```rust
let s = String::from("hello");
```

This kind of string _can_ be mutated:

```rust
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s);
```

A `String` can be mutated but not literals because of how these two types deal
with memory.

## Memory and Allocation

With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unknown at compile time,
to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we're done with
our `String`.

That first part is done by us: when we called `String::from`, its
implementation requests the memory it needs.

For the second part, in languages with a _garbage collector (GC)_, the GC keeps
track of and cleans up memory that isn't being used anymore, and we don't need
to think about it. In most languages without a GC, it's our responsibility to
identify when memory is no longer being used and to call code to explicitly
free it. We need to pair exactly one `allocate` with exactly one `free`.

Rust takes a different path: the memory is automatically returned once the
variable that owns it goes out of scope. When a variable goes out of scope,
Rust calls a special function called `drop` automatically.

In C++, this pattern of deallocating resources at the end of an item's lifetime
is sometimes called _Resource Acquisition Is Initialisation (RAII)_.

## Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust.

```rust
let x = 5;
let y = x;
```

The code above binds the value `5` to `x` and then makes a copy of the value in
`x` and binds it to `y`. Integers are simple values with a known fixed size so
they are pushed onto the stack.

Here's the `String` version.

```rust
let s1 = String::from("hello");
let s2 = s1;
```

A `String` is made up of three parts:

1. A pointer to the memory that holds the contents of the string,
2. A length, and
3. A capacity.

This group of data is stored on the stack. The contents of the string is stored
on the heap.

The length is how much memory, in bytes, the contents of the `String` is using.
The capacity is the total amount of memory, in bytes, that the `String` has
received from the allocator.

When we assigned `s1` to `s2`, the `String` data is copied (1., 2., and 3.).
The data on the heap that the pointer refers to is not copied.

Recall that when a variable goes out of scope, Rust automatically calls the
`drop` function and cleans up the heap memory for that variable. Since `s1` and
`s2` both point to the same data, so when they go out of scope, they will both
try to free the same memory. This is known as a _double free_ error and is a
memory safety bug, since freeing memory twice can lead to memory corruption,
which can potentially lead to security vulnerabilities.

To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as
no longer valid; this is known as a _move_ and we would say that `s1` was
_moved_ into `s2`. Therefore, Rust does not need to free anything when `s1`
goes out of scope. Only when `s2` goes out of scope will the memory be freed.

The following code will result in a compile error because Rust prevents you
from using the invalidated reference.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}"", world!", s1);
```

## Variables and Data Interacting with Clone

If we _do_ want to deeply copy the heap data of the `String`, not just the
stack data, we can use a common method called `clone`.

The following code works because the heap data is copied.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

## Stack-Only Data: Copy

Recall the following example from before; `x` is still valid.

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Types such as integers have a known size at compile time and are stored
entirely on the stack, so copies of the actual values are quick to make.
Therefore, there is no reason for invalidating `x` after we created `y`. Rust
has a special annotation called the `Copy` trait that can be used on types that
are stored on the stack. If a type implements the `Copy` trait, variables that
use it do not move, but are copied.

Rust won't let us annotate a type with `Copy` if they type, or any of its
parts, has implemented the `Drop` trait.

## Ownership and Functions

The mechanics of passing a value to a function are similar to those when
assigning a value to a variable. Passing a variable to a function will move or
copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function...
                                   // ... and so is no longer valid here

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // x would move into the function,
                                   // but i32 is Copy, so it's okay to still
                                   // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we try to use `s` after the call to `takes_ownership`, Rust will throw a
compile-time error.

## Return Values and Scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String { // gives_ownership will move its
                                 // return value into the function
                                 // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.

Rust has a feature for using a value without transferring ownership, called
_references_.

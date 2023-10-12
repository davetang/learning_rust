## Table of Contents

  - [Error Handling](#error-handling)
  - [Unrecoverable Errors with panic!](#unrecoverable-errors-with-panic)
  - [Using a panic! Backtrace](#using-a-panic-backtrace)

## Error Handling

Rust has a number of features for handling situations in which something goes
wrong. In many cases, Rust requires you to acknowledge the possibility of an
error and take some action before your code will compile. This requirement
makes your program more robust by ensuring that you'll discover errors and
handle them appropriately.

Rust groups errors into two major categories: _recoverable_ and _unrecoverable_
errors. For a recoverable error, such as a _file not found_ error, we most
likely just want to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array, and so we want to immediately stop the
program.

Most languages don't distinguish between these two kinds of errors and handle
both in the same way, using mechanisms such as exceptions. Rust does not have
exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
Unrecoverable error.

## Unrecoverable Errors with panic!

There are two ways to cause a panic in practice:

1. By taking an action that causes our code to panic (such as accessing an
   array past the end) or,

2. By explicitly calling the `panic!` macro.

By default, these panics will print a failure message, unwind, clean up the
stack, and quit. You can also have Rust display the call stack when a panic
occurs to make it easier to track down the source of the panic via an
environment variable.

Unwinding means Rust walks back up the stack and cleans up the data from each
function it encounters. However, this walking back and cleanup is a lot of
work. Rust allows you to choose the alternative of immediately _aborting_,
which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the
operating system. If your project requires that the resulting binary be as
small as possible, you can switch from unwinding to aborting upon a panic by
adding the following to the appropriate `[profile]` section in your
_Cargo.toml_ file.

```
[profile.release]
panic = 'abort'
```

## Using a panic! Backtrace

Here's an example of a `panic!` call from a library because of a bug in our
code.

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

In the above code, Rust will panic because using `[]` is supposed to return an
element but if you pass an invalid index, there's no element that Rust could
return.

In C, attempting to read beyond the end of a data structure is _undefined
behaviour_. You might get whatever is at the location in memory that would
correspond to that element in the data structure, even though the memory does
not belong to that structure. This is called a _buffer overread_ and can lead
to security vulnerabilities if an attacker is able to manipulate the index in
such a way as to read data they shouldn't be allowed to that is stored after
the data structure.

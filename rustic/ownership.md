## Table of Contents

- [Understanding Ownership](#understanding-ownership)
  - [The Stack and the Heap](#the-stack-and-the-heap)

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

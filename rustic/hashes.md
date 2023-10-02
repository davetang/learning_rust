## Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type
`V` using a _hashing function_, which determines how it places these keys and
values into memory.

One way to create an empty hash map is using `new` and adding elements with
`insert`.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type as each other, and all of
the values must have the same type.

We get a value out of the hash map by providing its key to the `get` method.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

Here, `score` will have the value that's associated with the Blue team, and the
result will be `10`. The `get` method returns an `Option<&V>`; if there's no
value for that key in the hash map, `get` will return `None`. This program
handles the `Option` by calling `copied` to get an `Option<i32>` rather than an
`Option<&i32>`, then `unwrap_or` to set `score` to zero if `scores` does not
have an entry for the key.

We can iterate over each key/value in a hash map in a `for` loop, which is
similar to what we do with vectors.

```rust
use std::collections::HashMap;

let mut score = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

## Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values.

```rust
use std::collections::HashMap;

let field_name = String::from("Favourite colour");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
```

The variables `field_name` and `field_value` are invalid after been moved into
the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won't be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid.

## Updating a Hash Map

When you want to change the data in a hash map, you decide how to handle the
case when a key already has a value assigned. You could replace the old value
with the new value, completely disregarding the old value. You could keep the
old value and ignore the new value, only adding the new value if the key
_doesn't_ already have a value. Or you could combine the old value and the new
value.

If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

## Adding a Key and Value Only If a Key Isn't Present

It's common to check whether a particular key already exists in the hash map
with a value then take the following actions: if the key does exist in the hash
map, the existing value should remain the same. If the key doesn't exist,
insert it and a value for it.

Hash maps have a special API for this called `entry`, which takes the key you
want to check as a parameter. The return value of the `entry` method is an enum
called `Entry` that represents a value that might or might not exist.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.

## Updating a Value Based on the Old Value

Another common use case for hash maps is to look up a key's value and then
update it based on the old value.

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```

The `split_whitespace` method returns an iterator over sub-slices, separated by
whitespace, of the value in `text`. The `or_insert` method returns a mutable
reference (`&mut V`) to the value for the specified key. Here we store that
mutable reference in the `count` variable, so in order to assign to that value,
we must first dereference `count` using the asterisk (`*`). The mutable
reference goes out of scope at the end of the `for` loop, so all of these
changes are safe and allowed by the borrowing rules.


## Hashing Functions

By default, `HashMap` uses a hashing function called _SipHash_ that can provide
resistance to Denial of Service (DoS) attacks involving hash tables. This is
not the fastest hashing algorithm available, but the trade-off for better
security that comes with the drop in performance is worth it. If you profile
your code and find that the default hash function is too slow for your
purposes, you can switch to another function by specifying a different hasher.
A _hasher_ is a type that implements the `BuildHasher` trait.

# README

Install [rustlings](https://github.com/rust-lang/rustlings), small exercises to
get you used to reading and writing Rust code!

```console
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
```

By default the installed files will be in the `rustlings` directory.

The exercises are sorted by topic and can be found in the subdirectory
rustlings/exercises/<topic>. For every topic there is an additional README file
with some resources to get you started on the topic.

The task is simple. Most exercises contain an error that keeps them from
compiling, and it's up to you to fix it! Some exercises are also run as tests,
but rustlings handles them all the same. To run the exercises in the
recommended order, execute:

```console
rustlings watch
```

If you ran the above command for the first time, the first exercise
(`exercises/intro/intro1.rs`) will be run. Edit `intro1.rs` accordingly in
another window and when you save it, `rustlings` will compile it. If you
successfully compile the exercise you move on to the next one. That's it!

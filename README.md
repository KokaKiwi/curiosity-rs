curiosity-rs
============

Build
-----

Just type `cargo build`

Run
---

Run the program with a crate entry file as argument.

```
$ target/curiosity src/lib.rs
Let declarations: 8
Function declarations: 2
```

Use the library
---------------

Just add the following lines to your `Cargo.toml` file:

```toml
[dependencies.curiosity]
git = "https://github.com/KokaKiwi/curiosity-rs"
```

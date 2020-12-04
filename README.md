# aoc-2020
Advent of Code 2020


# How to Contribute

Each day's challenged is arranged in folder, conveniently named `day-x`

To add your solution for a particular day. 
* Add a executable (bin) file with your name/userhandle on the `src` folder.
    - e.g. `touch day-1/src/{my_cool_name}.rs`
* Register your file as a binary in the `Cargo.toml` of the day.

```toml
[package]
name = "day-1"
version = "0.1.0"
authors = [""]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bin]]
name = "{my_cool_name}"
path = "src/{my_cool_name}.rs"
```

* To execute you binary, call `cargo` with `--bin {my_cool_name``}` in the `day-x` folder.
    - `cd day-1`
    - `cargo run --bin {my_cool_name}`

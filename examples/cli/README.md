# CLI example

`src/lib.rs` is tiny, it defines a struct to implement Solutions for.

`src/main.rs` contains the same code as the [cli example](../cli).

`day1.rs` contains a Solution to a "nonsense" Day 1 of Advent of Code.

Execute this example using `cargo run --example -- 1`.

This passes `1` as the only argument to this cli

This tells it to load input from a file for day1 and run the Day1 Solution with that input.

Try running the example passing the argument `2` or `3`.

You could extend this example by taking the year as an argument as well as the day.

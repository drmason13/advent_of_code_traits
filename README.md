[![Build Status][github-actions-badge]][github-actions] [![Latest Version][crates-io-badge]][crates-io]

# `impl Solution<Day25> for AdventOfCode2021`

## What is this?

This is [advent_of_code_traits][github], a minimal, flexible framework for implementing solutions to [Advent of Code] in Rust.

It takes a trait-based approach using const-generics.

## Experimental

This is already serviceable, but there will be frequent breaking changes as the traits are improved and refined.
The plan is to release a stable version in time for December 2021.


## Usage

Please see also the [examples](./examples/).

Implement traits with your solutions to each Day of Advent of Code.

### Import the `Solution` trait:

and optionally, consts:
```rust
use advent_of_code_traits::{days::*, Part1, Part2, Solution};
```

### Implement `Solution` for your struct.

```rust
pub struct AdventOfCode2020;

impl Solution<Day1, Part1> for AdventOfCode2020 {
    type Input = Vec<u32>;
    type Output = u32;

    fn parse(input: &str) -> anyhow::Result<u32> {
        // your parsing of part1 input here...
        input
            .lines()
            .map(|s| s.parse().expect("invalid integer"))
            .collect()
    }
    
    fn solve(input: &Vec<u32>) -> anyhow::Result<u32> {
        // your solution to part1 here...
        todo!()
    }
}
```

### Run from main.rs

Here comes the ugly part.
```rust
let input = std::fs::read_to_string("./input/2020/day1.txt").expect("failed to read input");
<AdventOfCode2020 as Solution<Day1>>::run(input);
```
This reads input from a file and passes it to your struct.
[Fully Qualified Syntax]
is required in order to disambiguate which day's Solution we are running.

Please refer to the [examples](./examples/) for more possibilities,
including parsing a different type for each Part and opting out of parsing entirely to work directly with the `&str`.

## How does this use const generics?

Because the `Solution` trait is generic over `const Day: u32` and `const Part: u32` you are free to implement them many times for the same struct.
The compiler will only yell at you if you implement them for the same Day and Part twice (as it should!).

`Day1` and `Part1` are used in the examples (because it looks awesome in my humble opinion). They are simply `1_u32`.

For example, `advent_of_code_traits::days` looks like this:

```
mod days {
    pub const Day1: u32 = 1;
    pub const Day2: u32 = 2;
    // ...
    pub const Day25: u32 = 25;
}
```

## Prior Art

I am very grateful for @gobanos' [cargo-aoc] which was a huge inspiration while creating this.

This crate is no match for the convenience or ease of use of [cargo-aoc].

Having said that, I hope it brings something new to the table (faster compile times perhaps?) and that others enjoy using this half as much as I enjoyed using [cargo-aoc].

I have used [cargo-aoc] for all of my Advent of Codes in Rust before 2021, and it is a brilliant, crazy use of procedural macros.

Thank you Gobanos! :)

## Contributing

Contributions are welcome, please see [CONTRIBUTING](./CONTRIBUTING.md)

Please also see [ARCHITECTURE](./ARCHITECTURE.md) for a guided tour of sorts of the code base.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in advent_of_code_traits by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

[crates-io]: https://crates.io/crates/advent_of_code_traits
[crates-io-badge]: https://img.shields.io/crates/v/advent_of_code_traits

[github]: https://github.com/drmason13/advent_of_code_traits
[github-actions]: https://github.com/drmason13/advent_of_code_traits/actions
[github-actions-badge]: https://github.com/drmason13/advent_of_code_traits/actions/workflows/github-actions.yml/badge.svg

[Advent of Code]: https://adventofcode.com
[Fully Qualified Syntax]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
[cargo-aoc]: https://github.com/gobanos/cargo-aoc

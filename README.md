[![Build Status][github-actions-badge]][github-actions] [![Latest Version][crates-io-badge]][crates-io]

# `impl Solution<'_, Day25, Part2> for AdventOfCode2021<Day25>`

## What is this?

This is [`advent_of_code_traits`](https://github.com/drmason13/advent_of_code_traits), a set of traits to implement solutions to Advent of Code in Rust.

It takes a trait-based approach using const-generics and autoderef specialization.

It's basically an excuse to play with rust's type system.

## Usage

Please see also the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples).

Implement traits with your solutions to each day of Advent of Code.

## Experimental

This is already serviceable, but there will be ~~frequent~~ breaking changes as the traits are improved and refined.
The "plan" is to release a stable version ~~in time for December 2021~~ when it's ready.

See the [Changelog](./CHANGELOG.md) for a current view of progress.

## Usage

Please see also the [examples](./examples/).

Implement traits with your solutions to each Day of Advent of Code.

### Import the machinery:

```rust
use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, run, Solution, SolutionRunner};
```

### Implement `Solution` for your struct.

```rust
pub struct AdventOfCode2021<const DAY: u32>;

impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
    type Input = Vec<u32>;
    type Output = u32;
    
    fn solve(&self, input: &Self::Input) -> Self::Output {
        // your solution to Part1 here...
    }
}
```
That's how we solve the solution given a nicely typed `Vec<u32>`, but Advent of Code gives us plaintext input.

So first we need to parse the input...

### Implement `ParseInput` for your struct

```rust
// ..continued from above

impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
    type Parsed = Vec<u32>; // <-- the input type fed to Solution::solve

    fn parse_input(&self, input: &'_ str) -> Self::Parsed {
        input
            .lines()
            .map(|s| s.parse().expect("invalid integer"))
            .collect()
    }
}
```

### Run from main.rs

Here comes the part where we actually run our solution!
```rust
let input = std::fs::read_to_string("./input/2021/day25.txt").expect("failed to read input");
run!(AdventOfCode2021::<Day25>, &input);
```
This reads input from a file and passes it to your struct to parse and then solve.
It will print the output of your solution (which must impl `Debug`).

[`run`] is currently a humble `macro_rules!` declarative macro and is *very* simple.
It's main purpose is to veil the use of [autoderef specialization].

Please refer to the [examples](https://github.com/drmason13/advent_of_code_traits/tree/v0.2.0/examples) for more demonstrations.

## How does this use const generics?

Because the `Solution` and `ParseInput` traits are generic over `const DAY: u32` you are free to implement them many times for the same struct.
The compiler will only yell at you if you implement them for the same DAY twice (as it should!).

`Day1` is used in the examples (because it looks awesome in my humble opinion). It is simply `1_u32`.

`advent_of_code_traits::days` looks like this:

```
mod days {
    pub const Day1: u32 = 1;
    pub const Day2: u32 = 2;
    // ...
    pub const Day25: u32 = 25;
}
```

`Solution` and `ParseInput` are also generic over `const PART: u8` which works very similarly to DAY.

## How does this use specialization

## Prior Art

I am very grateful for @gobanos' [cargo-aoc] which was a huge inspiration while creating this.

This crate is no match for the convenience or ease of use of [cargo-aoc].

Having said that, I hope it brings something new to the table (faster compile times perhaps?) and that others enjoy using this half as much as I enjoyed using [cargo-aoc].

I have used [cargo-aoc] for all of my Advent of Codes in Rust before 2021, and it is a brilliant, crazy use of procedural macros.

Thank you Gobanos! :)

## Contributing

Contributions are welcome, please see [CONTRIBUTING](./CONTRIBUTING.md)

Please also see [ARCHITECTURE](./ARCHITECTURE.md) for a guided tour of sorts of the code base.

I might take a long time to merge/release your contributions, I'm still inexperienced at the whole open source mainenance thing.
I am grateful for them though.

### Thank you so much to everyone who has helped this project so far:

* [hui.liu hulufei](https://github.com/hulufei)

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
[autoderef specialization]: http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
[cargo-aoc]: https://github.com/gobanos/cargo-aoc
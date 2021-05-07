# `impl Solution<Day25> for AdventOfCode2021`

## What is this?

This is [`advent_of_code_traits`](https://github.com/drmason13/advent_of_code_traits), a minimal, flexible framework for implementing solutions to Advent of Code in Rust.

It takes a trait-based approach using const-generics.

## Usage

Please see also the [examples](./examples/).

Implement traits with your solutions to each Day of Advent of Code.

### Import the traits:

```rust
use advent_of_code_traits::{ParseInput, Solution};
```
and optionally, consts:
```rust
use advent_of_code_traits::{days::*, Part1, Part2};
```

### Implement `Solution` for your struct.

```rust
pub struct AdventOfCode2020;

impl Solution<Day1> for AdventOfCode2020 {
    type Part1Output = u32;
    type Part2Output = u32;

    fn part1(input: &Vec<u32>) -> u32 {
        // your solution to part1 here...
        todo!()
    }
    
    fn part2(input: &Vec<u32>) -> u32 {
        // your solution to part2 here...
        todo!()
    }
}
```

"But where does `Vec<u32>` come from?", you ask.

Well spotted, eagle-eyed reader!

That comes from your implementation of `ParseInput`.

### Implement `ParseInput` for your struct

```rust
// ..continued from above

impl ParseInput<Day1> for AdventOfCode2020 {
    type Parsed = Vec<u32>; // <-- this will be the input to both part1 and part2 for Solution<Day1>

    fn parse_input(input: &str) -> Self::Parsed {
        input
            .lines()
            .map(|s| s.parse().expect("invalid integer"))
            .collect()
    }
}
```

Please refer to the examples for more possibilities,
including parsing a different type for each Part and opting out of parsing entirely to work directly with the `&str`.

### Run from main.rs

Here comes the ugly part.
```rust
let input = std::fs::read_to_string("./input/2020/day1.txt").expect("failed to read input");
<AdventOfCode2020 as Solution<Day1>>::run(input);
```
This reads input from a file and passes it to your struct.
[Fully Qualified Syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name)
is required in order to disambiguate which day's Solution we are running.

## How does this use const generics?

Because the `Solution` and `ParseInput` traits are generic over `const Day: u32` you are free to implement them many times for the same struct.
The compiler will only yell at you if you implement them for the same Day twice (as it should!).

`Day1` is used in the examples (because it looks awesome in my humble opinion).

It is literally just `1_u32`. `advent_of_code_traits::days` looks like this:

```
mod days {
    pub const Day1: u32 = 1;
    pub const Day2: u32 = 2;
    // ...
    pub const Day25: u32 = 25;
}
```

## Prior Art

I am very grateful for @gobanos' [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) which was a huge inspiration while creating this.

This crate is no match for the convenience or ease of use of `cargo-aoc`.

Having said that, I hope it brings something new to the table (faster compile times perhaps?) and that others enjoy using this half as much as I enjoyed using `cargo-aoc`.

I have used the cargo-aoc plugin for all of my Advent of Codes in Rust before 2021, and it is a brilliant, crazy use of procedural macros to make our lives easier. 

Thank you Gobanos! :)

## Contributing

This currently lacks support for **Error Handling** in the trait definitions.

I'd greatly appreciate contributions of ideas and/or code here, it has me a bit stumped but I'm sure it can be done!

I'm very reluctant to release a 1.0 without big improvements here.

Please also see [ARCHITECTURE.md](./ARCHITECTURE.md) for a guided tour of sorts of the code base.

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

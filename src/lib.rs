//! # `impl Solution<Day25> for AdventOfCode2021`
//!
//! ## What is this?
//!
//! This is [`advent_of_code_traits`](https://github.com/drmason13/advent_of_code_traits), a minimal, flexible framework for in implementing solutions to Advent of Code in Rust.
//!
//! It takes a trait-based approach using const-generics.
//!
//! ## Usage
//!
//! Please see also the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples).
//!
//! Implement traits with your solutions to each Day of Advent of Code.
//!
//! ### Import the traits:
//!
//! ```no_run
//! use advent_of_code_traits::{days::Day1, ParseInput, Solution};
//! ```
//!
//! ### Implement [`Solution`] for your struct.
//!
//! ```no_run
//! # use advent_of_code_traits::{days::Day1, ParseInput, Solution};
//! pub struct AdventOfCode2020;
//!
//! impl Solution<Day1> for AdventOfCode2020 {
//!     type Part1Output = u32;
//!     type Part2Output = u32;
//!
//!     fn part1(input: &Vec<u32>) -> u32 {
//!         // your solution to part1 here...
//! #       1
//!     }
//!
//!     fn part2(input: &Vec<u32>) -> u32 {
//!         // your solution to part2 here...
//! #       2
//!     }
//! }
//!
//! # impl ParseInput<Day1> for AdventOfCode2020 {
//! #     type Parsed = Vec<u32>; // <-- the input to both part1 and part2 for Solution<Day1>
//! #
//! #     fn parse_input(input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! ```
//!
//! This is completely valid rust code, don't you like the way it reads?
//!
//! "But where does `input: Vec<u32>` come from?", you ask.
//!
//! Well spotted, eagle-eyed reader!
//!
//! That comes from an implementation of `ParseInput`.
//!
//! ### Implement [`ParseInput`] for your struct
//!
//! ```
//! # use advent_of_code_traits::{days::Day1, ParseInput, Solution};
//! # pub struct AdventOfCode2020;
//! #
//! # impl Solution<Day1> for AdventOfCode2020 {
//! #     type Part1Output = u32;
//! #     type Part2Output = u32;
//! #
//! #     fn part1(input: &Vec<u32>) -> u32 {
//! #         // your solution to part1 here...
//! #         1
//! #     }
//! #
//! #     fn part2(input: &Vec<u32>) -> u32 {
//! #         // your solution to part2 here...
//! #         2
//! #     }
//! # }
//! // ..continued from above
//!
//! impl ParseInput<Day1> for AdventOfCode2020 {
//!     type Parsed = Vec<u32>; // <-- the input to both part1 and part2 for Solution<Day1>
//!
//!     fn parse_input(input: &str) -> Self::Parsed {
//!         input
//!             .lines()
//!             .map(|s| s.parse().expect("invalid integer"))
//!             .collect()
//!     }
//! }
//! # assert_eq!(1, <AdventOfCode2020 as Solution<Day1>>::part1(&vec![1, 2, 3]));
//! # assert_eq!(2, <AdventOfCode2020 as Solution<Day1>>::part2(&vec![1, 2, 3]));
//! # assert_eq!(vec![1, 2, 3], <AdventOfCode2020 as ParseInput<Day1>>::parse_input("1\n2\n3"));
//! ```
//!
//! Please refer to the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples) for more possibilities,
//! including parsing a different type for each Part and opting out of parsing entirely to work directly with the `&str`.
//!
//! ### Run from `main.rs`
//!
//! Here comes the ugly part.
//! ```no_run
//! # use advent_of_code_traits::{days::Day1, ParseInput, Solution};
//! # pub struct AdventOfCode2020;
//! #
//! # impl Solution<Day1> for AdventOfCode2020 {
//! #     type Part1Output = u32;
//! #     type Part2Output = u32;
//! #
//! #     fn part1(input: &Vec<u32>) -> u32 {
//! #         // your solution to part1 here...
//! #         1
//! #     }
//! #
//! #     fn part2(input: &Vec<u32>) -> u32 {
//! #         // your solution to part2 here...
//! #         2
//! #     }
//! # }
//! # impl ParseInput<Day1> for AdventOfCode2020 {
//! #     type Parsed = Vec<u32>; // <-- the input to both part1 and part2 for Solution<Day1>
//! #
//! #     fn parse_input(input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! let input = std::fs::read_to_string("./input/2020/day1.txt").expect("failed to read input");
//! <AdventOfCode2020 as Solution<Day1>>::run(&input);
//! ```
//! This reads input from a file and passes it to your struct.
//! [Fully Qualified Syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name)
//! is required in order to disambiguate which day's Solution we are running.

#![allow(non_upper_case_globals)]

use std::fmt::Display;

pub mod days;
/// Constant for part1 of each day.
/// See also [`ParseEachInput`].
pub const Part1: u8 = 1;
/// Constant for part2 of each day.
/// See also [`ParseEachInput`].
pub const Part2: u8 = 2;

pub use anyhow;

pub trait Solution<const Day: u8, const Part: u8> {
    type Input;
    type Output: Display;

    #[allow(unused_variables)]
    fn parse(&self, input: &str) -> anyhow::Result<Self::Input> {
        panic!(
            "No parsing implemented for Part1, Day{}.\n\
            The `parse` method causes this panic by default.\n\
            You may want to implement `parse` yourself or implement `run` yourself and not call this method",
            Day
        )
    }

    #[allow(unused_variables)]
    fn solve(&self, input: &Self::Input) -> anyhow::Result<Self::Output> {
        panic!(
            "No solution implemented for Part1, Day{}.\n\
            The `solve` method triggers this panic by default.\n\
            Either implement `solve` yourself or implement `run` yourself and not call this method.",
            Day
        )
    }

    fn run(&self, input: &str) -> anyhow::Result<()> {
        let output = self.solve(&self.parse(input)?)?;
        self.report(&output)?;
        Ok(())
    }

    fn report(&self, output: &impl Display) -> anyhow::Result<()> {
        println!(
            "Day {} Part {}:\n\
            {}",
            Day, Part, output
        );
        Ok(())
    }
}

// pub trait Runner<const Day: u8, const Part: u8>: Solution<Day, Part> {
// }

// impl<T, const Day: u8, const Part: u8> Runner<Day, Part> for T where T: Solution<Day, Part> {}

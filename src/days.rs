//! Constants for all 25 days of Advent
//!
//! ## Examples
//!
//! In your module containing a solution for day 1
//! ```no_run
//! use advent_of_code_traits::days::Day1;
//! ```
//!
//! They are just plain `u32`s.
//! ```
//! # use advent_of_code_traits::days::Day1;
//! assert_eq!(1, Day1);
//! ```
//!
//! You don't have to use these consts at all if you don't want to
//! ```no_run
//! # use advent_of_code_traits::{ParseInput, Solution};
//! # pub struct AdventOfCode2020;
//! impl Solution<2> for AdventOfCode2020 {
//!     # type Part1Output = u32;
//!     # type Part2Output = u32;
//!     // this works the same as `Day2`
//!     # fn part1(input: &()) -> u32 {1}
//!     # fn part2(input: &()) -> u32 {2}
//! }
//! # impl ParseInput<2> for AdventOfCode2020 {
//! #     type Parsed = ();
//! #
//! #     fn parse_input(input: &str) -> Self::Parsed {
//! #         todo!()
//! #     }
//! # }
//! ```

include!(concat!(env!("OUT_DIR"), "/const_days.rs"));

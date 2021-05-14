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
//! # use advent_of_code_traits::{Part1, Part2, Solution};
//! # pub struct AdventOfCode2020;
//! impl Solution<2, Part1> for AdventOfCode2020 {
//!     // this works the same as `Day2`
//!     # type Input = u32;
//!     # type Output = u32;
//!     # fn parse(&self, input: &str) -> anyhow::Result<u32> {Ok(1)}
//!     # fn solve(&self, input: &u32) -> anyhow::Result<u32> {Ok(2)}
//! }
//! ```

include!(concat!(env!("OUT_DIR"), "/const_days.rs"));

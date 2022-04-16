//! Constants for all 25 days of Advent
//!
//! ## Examples
//!
//! In your module containing a solution for day 1
//! ```
//! use advent_of_code_traits::days::Day1;
//! ```
//!
//! They are just plain `u32`s.
//! ```
//! # use advent_of_code_traits::days::Day1;
//! assert_eq!(1_u32, Day1);
//! ```
//!
//! They pair well with the [`Part1`](super::Part1) and [`Part2`](super::Part2) consts
//! which are just 1 and 2, but u8 instead of u32 to help avoid mixing them up by accident.
//!
//! You don't have to use these consts at all if you don't want to.
//! ```no_run
//! # use advent_of_code_traits::{ParseInput, Solution};
//! # pub struct Problem;
//! // this works the same as using `Solution<Day2, Part1>`
//! impl Solution<'_, 2, 1> for Problem {
//!     # type Input = u32;
//!     # type Output = u32;
//!     # fn solve(&self, input: &Self::Input) -> Self::Output { 1 }
//!     // ...
//! }
//! # impl ParseInput<'_, 2, 1> for Problem {
//! #     type Parsed = ();
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         todo!()
//! #     }
//! # }
//! ```
#![allow(non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/const_days.rs"));

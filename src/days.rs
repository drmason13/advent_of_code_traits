//! Constants for all 25 days of Advent
//!
//! ## Examples
//!
//! In your module containing a solution for day 1
//! ```
//! use advent_of_code_traits::days::Day1;
//! ```
//!
//! These are just plain `u32`s.
//! ```
//! # use advent_of_code_traits::days::Day1;
//! assert_eq!(1, Day1);
//! ```
//!
//! You don't have to use these consts where it doesn't make sense to
//! ```
//!
//! ```

include!(concat!(env!("OUT_DIR"), "/const_days.rs"));

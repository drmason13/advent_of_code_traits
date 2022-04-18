//! # `impl Solution<'_, Day25, Part2> for AdventOfCode2021<Day25>`
//!
//! ## What is this?
//!
//! This is [`advent_of_code_traits`](https://github.com/drmason13/advent_of_code_traits), a set of traits to implement solutions to Advent of Code in Rust.
//!
//! It takes a trait-based approach using const-generics and autoderef specialization.
//!
//! It's basically an excuse to play with rust's type system.
//!
//! ## Usage
//!
//! Please see also the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples).
//!
//! Implement traits with your solutions to each day of Advent of Code.
//!
//! ### Import the machinery:
//!
//! ```
//! use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! ```
//!
//! ### Implement [`Solution`] for your struct.
//!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! pub struct AdventOfCode2021<const DAY: u32>;
//!
//! impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//!     type Input = Vec<u32>;
//!     type Output = u32;
//!
//!     fn solve(&self, input: &Self::Input) -> Self::Output {
//!         // your solution to Part1 here...
//! #       1
//!     }
//! }
//!
//! # impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Parsed = Vec<u32>; // <-- the input to both PartOne and PartTwo for Solution<Day1>
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! # impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
//! ```
//!
//! That's how we solve the solution given a nicely typed `Vec<u32>`, but Advent of Code gives us plaintext input.
//!
//! So first we need to parse the input...
//!
//! ### Implement [`ParseInput`] for your struct
//!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! #
//! # impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Input = Vec<u32>;
//! #     type Output = u32;
//! #
//! #     fn solve(&self, input: &Vec<u32>) -> u32 {
//! #         // your solution to Part1 here...
//! #         1
//! #     }
//! # }
//! // ..continued from above
//!
//! impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//!     type Parsed = Vec<u32>; // <-- the input type fed to Solution::solve
//!
//!     fn parse_input(&self, input: &'_ str) -> Self::Parsed {
//!         input
//!             .lines()
//!             .map(|s| s.parse().expect("invalid integer"))
//!             .collect()
//!     }
//! }
//! # let input = "1\n2\n3";
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! # impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
//! # let prb = AdventOfCode2021::<Day25>;
//! # let parsed = &prb.parse_input(&input);
//! # let ans = prb.solve(&parsed);
//! # assert_eq!(1, ans);
//! ```
//!
//! ### Mark Part2 as missing
//!
//! To run only Part1 of a day of Advent of Code, you currently need to impl `MissingPartTwo` to help disambiguate the specialization:
//! ```no_run
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! ```
//!
//! If you don't do this (and haven't implemented Solution for Part2) you'll see an error like:
//! ```text
//! the method `run` exists for reference `&&&AdventOfCode2021<25_u32>`, but its trait bounds were not satisfied
//! the following trait bounds were not satisfied:
//! `AdventOfCode2021<25_u32>: MissingPartTwo<25_u32>`
//! which is required by `AdventOfCode2021<25_u32>: SolutionRunner<25_u32, 1_u16>`rustcE0599
//! ```
//!
//! ### Implement [`Reporter`] for your struct
//!
//! Let's just go with the default implmentation for now.
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
//! ```
//!
//! Please refer to the [examples](https://github.com/drmason13/advent_of_code_traits/tree/main/examples) for more demonstrations.
//!
//! ### Run from `main.rs`
//!
//! Here comes the part where we actually run our solution!
//! ```
//! # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
//! # pub struct AdventOfCode2021<const DAY: u32>;
//! #
//! # impl Solution<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Input = Vec<u32>;
//! #     type Output = u32;
//! #
//! #     fn solve(&self, input: &Vec<u32>) -> u32 {
//! #         // your solution to Part1 here...
//! #         1
//! #     }
//! # }
//! # impl ParseInput<'_, Day25, Part1> for AdventOfCode2021<Day25> {
//! #     type Parsed = Vec<u32>; // <-- the input to both PartOne and PartTwo for Solution<Day1>
//! #
//! #     fn parse_input(&self, input: &str) -> Self::Parsed {
//! #         input
//! #             .lines()
//! #             .map(|s| s.parse().expect("invalid integer"))
//! #             .collect()
//! #     }
//! # }
//! # impl MissingPartTwo<Day25> for AdventOfCode2021<Day25> {}
//! # impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
//! # // for test purposes, circumvent the example code
//! # if false {
//! let input = std::fs::read_to_string("./input/2021/day25.txt").expect("failed to read input");
//! # }
//! # let input = "1\n2\n3";
//! run!(AdventOfCode2021::<Day25>, &input);
//! ```
//! This reads input from a file and passes it to your struct to parse and then solve.
//! It will print the output of your solution (which must impl Debug).
//!
//! [`run`] is currently a humble `macro_rules!` declarative macro and is *very* simple.
//! It's main purpose is to veil the use of autoderef [`specialization`].
use std::fmt::Debug;

pub mod days;

#[allow(non_upper_case_globals)]
pub const Part1: u8 = 1;
#[allow(non_upper_case_globals)]
pub const Part2: u8 = 2;

/// Implement this trait with your solution to the Advent of Code problem for a particular day and part.
///
/// Remember: Day, then Part.
///
/// The compiler will complain about u32 (days) vs u8 (parts) if you mix this up.
pub trait Solution<'a, const DAY: u32, const PART: u8> {
    type Input;
    type Output: Debug;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;
}

/// Implement this trait to parse the raw input into a more useful type for your [`Solution`] to use as input.
///
/// See [`Solution::Input`]
pub trait ParseInput<'a, const DAY: u32, const PART: u8> {
    type Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed;
}

/// Import this trait to run your advent of code solutions once they implement [`Solution`].
///
/// This trait doesn't need to be implemented outside of this crate.
///
/// Blanket implementations are provided that specialize if your solutions share a parsed input type or if [`MissingPartTwo`] is implemented.
pub trait SolutionRunner<'a, const DAY: u32, const IMPL: u16> {
    fn run(&'a self, input: &'a str);
}

/// The [`run`] macro expands
/// ```ignore
/// run!(AdventOfCode2021::<Day25>, &input);
/// ```
/// to
/// ```ignore
/// {
///     let problem = AdventOfCode2021::<Day25>;
///     (&&&problem).run(&input)
/// }
/// ```
/// and that's it!
///
/// What's with all the `&`s? It's autoderef specialization, see [`specialization`].
#[macro_export]
macro_rules! run {
    ($day: expr, $input: expr) => {{
        let problem = $day;
        (&&&problem).run($input)
    }};
}

/// [`MissingPartTwo`] is a marker trait to tell the compiler that your struct doesn't impl Solution for Part2.
///
/// Implementing this is required in order to run only Part1 using [`SolutionRunner::run`] without specifying which SolutionRunner impl to use manually.
///
/// Why? It's to guide the specialization by ensuring that each impl is unique. See [`specialization`] for more details.
/// ```
/// # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
/// struct AdventOfCode2021<const DAY: u32>;
///
/// impl<'a> Solution<'a, Day1, Part1> for AdventOfCode2021<Day1> {
///     // your solution to part 1
/// #     type Input = u32; type Output = u32;
/// #     fn solve(&'a self, _input: &Self::Input) -> Self::Output { 1 }
/// }
/// # impl<'a> ParseInput<'a, Day1, Part1> for AdventOfCode2021::<Day1> {
/// #     type Parsed = u32;
/// #     fn parse_input(&'a self, input: &'a str) -> Self::Parsed { 1 }
/// # }
/// # impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
///
/// // add this to be able to .run() AdventOfCode2021::<Day25> without an implemention for Part2
/// impl MissingPartTwo<Day1> for AdventOfCode2021<Day1> {}
///
/// # let input: &'static str = "fake input";
/// # let problem = AdventOfCode2021::<Day1>;
/// // ...
/// problem.run(&input);
/// ```
pub trait MissingPartTwo<const DAY: u32> {}

/// [`Reporter`] provides methods to display output (or report) to the user what's happening throughout the run.
///
/// Each method is an "event" to be called by [`SolutionRunner`] at the appropriate time.
///
/// This trait must be implemented by the user, but every method has a default so it is quite easy!
/// ```
/// # use advent_of_code_traits::{days::*, MissingPartTwo, Part1, Part2, ParseInput, Reporter, run, Solution, SolutionRunner};
/// # pub struct AdventOfCode2021<const DAY: u32>;
/// impl<const DAY: u32> Reporter for AdventOfCode2021<DAY> {}
/// ```
///
/// The default implementation simply prints to stdout, run the cli example to see the format for yourself.
///
/// I expect it would be possible to implement benchmarking, writing to a file, and/or anything else you can imagine
/// in your own implementations of this trait. The signatures take `&self`, so you might want to use interior mutability
/// or some external mechanism to track state between runs (if you want to). This is left as an exercise for the reader.
pub trait Reporter {
    fn answer(&self, _day: u32, part: u8, answer: impl Debug) {
        println!("part {part}: {answer:?}");
    }

    fn parsing(&self, _day: u32, _part: u8) {}
    fn parsed(&self, _day: u32, _part: u8) {}

    fn solving(&self, _day: u32, _part: u8) {}
    fn solved(&self, _day: u32, _part: u8) {}

    fn start_day(&self, day: u32) {
        println!("Day {day:02}");
    }
    fn end_day(&self, _day: u32) {
        println!("---");
    }
}

pub mod specialization;

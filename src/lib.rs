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

pub mod days;
/// Constant for part1 of each day.
/// See also [`ParseEachInput`].
pub const Part1: u32 = 1;
/// Constant for part2 of each day.
/// See also [`ParseEachInput`].
pub const Part2: u32 = 2;

/// Implement the [`Solution`] trait for each day of Advent of Code for your struct(s).
///
/// Each day is a unique implementation, implement each on any struct you like.
///
/// ## Example
///
/// ```
/// # use advent_of_code_traits::{days::Day1, ParseInput, Solution};
/// pub struct AdventOfCode2020;
///
/// impl Solution<Day1> for AdventOfCode2020 {
///     type Part1Output = u32;
///     type Part2Output = u32;
///     fn part1(input: &Vec<u32>) -> u32 {
///         // your solution to part1 here...
/// #       1
///     }
///
///     fn part2(input: &Vec<u32>) -> u32 {
///         // your solution to part2 here...
/// #       2
///     }
/// }
/// #
/// # impl ParseInput<Day1> for AdventOfCode2020 {
/// #     type Parsed = Vec<u32>; // <-- the input to both part1 and part2 for Solution<Day1>
/// #
/// #     fn parse_input(input: &str) -> Self::Parsed {
/// #         input
/// #             .lines()
/// #             .map(|s| s.parse().expect("invalid integer"))
/// #             .collect()
/// #     }
/// # }
/// ```
pub trait Solution<const Day: u32>:
    ParseEachInput<Day, Part1> + ParseEachInput<Day, Part2>
{
    /// The type output by [`Solution::part1`]
    /// This must implement [`Display`][::std::fmt::Display] so that we can print it
    type Part1Output: std::fmt::Display;
    /// The type output by [`Solution::part2`]
    /// This must implement [`Display`][::std::fmt::Display] so that we can print it
    type Part2Output: std::fmt::Display;

    fn part1(input: &<Self as ParseEachInput<Day, Part1>>::Parsed) -> Self::Part1Output;
    fn part2(input: &<Self as ParseEachInput<Day, Part2>>::Parsed) -> Self::Part2Output;

    /// The default implementation of run will:
    /// * parse your input for each part
    /// * call `part1` and `part2` with their parsed inputs.
    /// * Print a short summary to display the output
    ///
    /// You can provide your own implementation of this method to change this deafult behaviour.
    ///
    /// ## Example
    ///
    /// ```
    /// # use advent_of_code_traits::{days::Day1, ParseInput, Solution};
    /// pub struct AdventOfCode2020;
    ///
    /// impl Solution<Day1> for AdventOfCode2020 {
    ///     type Part1Output = u32;
    ///     type Part2Output = u32;
    ///     fn part1(input: &Vec<u32>) -> u32 {
    ///         // your solution to part1 here...
    /// #       1
    ///     }
    ///
    ///     fn part2(input: &Vec<u32>) -> u32 {
    ///         // your solution to part2 here...
    /// #       2
    ///     }
    ///
    ///     fn run(input: &str) {
    ///         let shared_parsed_input = <Self as ParseInput<Day1>>::parse_input(input);
    ///
    ///         let part1_output = Self::part1(&shared_parsed_input);
    ///         let part2_output = Self::part2(&shared_parsed_input);
    ///
    ///         // maybe you prefer a single line output?
    ///         println!("Day{}: {} - {}", Day1, part1_output, part2_output);
    ///     }
    ///
    /// }
    /// # impl ParseInput<Day1> for AdventOfCode2020 {
    /// #     type Parsed = Vec<u32>; // <-- the input to both part1 and part2 for Solution<Day1>
    /// #
    /// #     fn parse_input(input: &str) -> Self::Parsed {
    /// #         input
    /// #             .lines()
    /// #             .map(|s| s.parse().expect("invalid integer"))
    /// #             .collect()
    /// #     }
    /// # }
    /// ```
    fn run(input: &str) {
        let part1_parsed_input = <Self as ParseEachInput<Day, Part1>>::parse_input(input);
        let part2_parsed_input = <Self as ParseEachInput<Day, Part2>>::parse_input(input);

        let part1_output = Self::part1(&part1_parsed_input);
        let part2_output = Self::part2(&part2_parsed_input);

        // TODO: extract printing behaviour into a report or summary method with a default implementation
        println!(
            "Day {0}, Part 1\n\
            {1}\n\n\
            Day {0}, Part 2\n\
            {2}",
            Day, part1_output, part2_output
        );
    }
}

/// Implement this trait if you need a different input for each part of a day.
///
/// This trait is generic over both day and part.
///
/// See also [`ParseInput`] which you should prefer implementing to use the *same* input type for each part of a day.
///
/// ## Example Usage
///
/// ```
/// # use std::collections::HashMap;
/// use advent_of_code_traits::{days::Day2, Part1, Part2, ParseEachInput};
/// pub struct AdventOfCode2020;
///
/// impl ParseEachInput<Day2, Part1> for AdventOfCode2020 {
///     type Parsed = Vec<u32>;
///
///     fn parse_input(_input: &str) -> Self::Parsed {
///         // parse your input for _part1_
///         // ...
///         // let's just cheat for demonstration purposes
///         vec![1]
///     }
/// }
///
/// impl ParseEachInput<Day2, Part2> for AdventOfCode2020 {
///     type Parsed = HashMap<String, u32>;
///
///     fn parse_input(_input: &str) -> Self::Parsed {
///         // parse your input for _part2_
///         // ...
///         // let's just cheat for demonstration purposes
///         let mut hashmap = HashMap::new();
///         hashmap.insert("A".into(), 2);
///         hashmap
///     }
/// }
///
/// let part1_input = <AdventOfCode2020 as ParseEachInput<Day2, Part1>>::parse_input("input");
/// let part2_input = <AdventOfCode2020 as ParseEachInput<Day2, Part2>>::parse_input("input");
/// assert_eq!(vec![1], part1_input);
/// assert_eq!(Some(&2), part2_input.values().next());
/// ```
pub trait ParseEachInput<const Day: u32, const Part: u32> {
    /// The type that you want your [`Solution`] code to receive for a particular part of a day.
    type Parsed;

    /// See [`ParseInput::parse_input`]
    fn parse_input(input: &str) -> Self::Parsed;
}

/// Implement this trait to parse the the day's input into a type.
///
/// This trait is generic over day.
///
/// See [`ParseEachInput`] if you want to use a *different* input type for each part of a day.
///
/// ## Example Usage
///
/// ```
/// use advent_of_code_traits::{days::Day1, Part1, Part2, ParseInput};
/// pub struct AdventOfCode2020;
///
/// impl ParseInput<Day1> for AdventOfCode2020 {
///     type Parsed = Vec<usize>;
///
///     fn parse_input(input: &str) -> Self::Parsed {
///         // parse your input for day 1
///         input.lines()
///             .map(|n| n.len())
///             .collect()
///     }
/// }
///
/// let part1_input = <AdventOfCode2020 as ParseInput<Day1>>::parse_input("input");
/// let part2_input = <AdventOfCode2020 as ParseInput<Day1>>::parse_input("input");
///
/// // both parts get the same input
/// assert_eq!(part1_input, part2_input);
/// ```
pub trait ParseInput<const Day: u32> {
    /// The type that you want your [`Solution`] code to receive
    type Parsed;

    /// This function receives the entire input file as a &str slice
    /// and must return a [`Self::Parsed`]
    fn parse_input(input: &str) -> Self::Parsed;
}

impl<T, const Day: u32> ParseEachInput<Day, Part1> for T
where
    T: ParseInput<Day>,
{
    type Parsed = T::Parsed;
    fn parse_input(input: &str) -> Self::Parsed {
        <Self as ParseInput<Day>>::parse_input(input)
    }
}

impl<T, const Day: u32> ParseEachInput<Day, Part2> for T
where
    T: ParseInput<Day>,
{
    type Parsed = T::Parsed;
    fn parse_input(input: &str) -> Self::Parsed {
        <Self as ParseInput<Day>>::parse_input(input)
    }
}

/// Conveniently running individual days
///
/// ## Example Usage
///
/// ```
/// use advent_of_code_traits::{days::Day1, Solution, ParseInput, run};
///
/// pub struct AdventOfCode2020;
///
/// impl ParseInput<Day1> for AdventOfCode2020 {
///     type Parsed = Vec<u32>;
///
///     fn parse_input(input: &str) -> Self::Parsed {
///         vec![1, 2, 3]
///     }
/// }
///
/// impl Solution<Day1> for AdventOfCode2020 {
///     type Part1Output = usize;
///     type Part2Output = String;
///
///     fn part1(input: &Vec<u32>) -> Self::Part1Output {
///         input.len()
///     }
///
///     fn part2(input: &Vec<u32>) -> Self::Part2Output {
///         String::from("...")
///     }
/// }
///
/// let input = "aoc input strings";
///
/// run!(AdventOfCode2020, Day1, &input)
/// ```
#[macro_export]
macro_rules! run {
    ($aoc: ty, $day: ty, $input: expr) => {
        <$aoc as Solution<$day>>::run($input)
    };
}

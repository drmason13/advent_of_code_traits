use advent_of_code_traits::{days::Day3, ParseInput, Part1, Solution};

use super::AdventOfCode2021;

impl<'a> ParseInput<'a, Day3, Part1> for AdventOfCode2021<Day3> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day3, Part1> for AdventOfCode2021<Day3> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().map(|n| n + 10).product()
    }
}

impl advent_of_code_traits::MissingPartTwo<Day3> for AdventOfCode2021<Day3> {}

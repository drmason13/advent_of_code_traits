use advent_of_code_traits::{days::Day2, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl<'a> ParseInput<'a, Day2, Part1> for AdventOfCode2021<Day2> {
    type Parsed = usize;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input.len()
    }
}

impl<'a> Solution<'a, Day2, Part1> for AdventOfCode2021<Day2> {
    type Input = usize;
    type Output = String;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.to_string()
    }
}

impl<'a> Solution<'a, Day2, Part2> for AdventOfCode2021<Day2> {
    type Input = usize;
    type Output = usize;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input * 10
    }
}

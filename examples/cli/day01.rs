use advent_of_code_traits::{days::Day1, ParseInput, Part1, Part2, Solution};

use super::AdventOfCode2021;

impl<'a> ParseInput<'a, Day1, Part1> for AdventOfCode2021<Day1> {
    type Parsed = Vec<u32>;

    fn parse_input(&'a self, _input: &'a str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl<'a> Solution<'a, Day1, Part1> for AdventOfCode2021<Day1> {
    type Input = Vec<u32>;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.iter().sum()
    }
}

impl<'a> ParseInput<'a, Day1, Part2> for AdventOfCode2021<Day1> {
    type Parsed = &'a str;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        input
    }
}

impl<'a> Solution<'a, Day1, Part2> for AdventOfCode2021<Day1> {
    type Input = &'a str;
    type Output = u32;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        input.len() as u32
    }
}

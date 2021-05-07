#![allow(unused_variables)]
use crate::AdventOfCode2020;
use advent_of_code_traits::{days::Day1, ParseInput, Solution};

impl ParseInput<Day1> for AdventOfCode2020 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        vec![1, 2, 3]
    }
}

impl Solution<Day1> for AdventOfCode2020 {
    type Part1Output = usize;
    type Part2Output = String;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        input.len()
    }

    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        input.iter().fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        })
    }
}

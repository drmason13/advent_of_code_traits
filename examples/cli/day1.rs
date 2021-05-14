#![allow(unused_variables)]
use crate::AdventOfCode2020;
use advent_of_code_traits::{days::Day1, Part1, Part2, Solution};

impl Solution<Day1, Part1> for AdventOfCode2020 {
    type Input = Vec<u32>;
    type Output = usize;

    fn parse(&self, input: &str) -> anyhow::Result<Self::Input> {
        Ok(vec![1, 2, 3])
    }

    fn solve(&self, input: &Vec<u32>) -> anyhow::Result<Self::Output> {
        Ok(input.len())
    }
}

impl Solution<Day1, Part2> for AdventOfCode2020 {
    type Input = Vec<u32>;
    type Output = String;

    fn parse(&self, input: &str) -> anyhow::Result<Self::Input> {
        Ok(vec![1, 2, 3])
    }

    fn solve(&self, input: &Vec<u32>) -> anyhow::Result<Self::Output> {
        Ok(input.iter().fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc
        }))
    }
}

use advent_of_code_traits::{days::Day2, ParseEachInput, Part1, Part2, Solution};
use std::collections::HashMap;

struct AdventOfCode2020;

impl ParseEachInput<Day2, Part1> for AdventOfCode2020 {
    type Parsed = Vec<u32>;

    fn parse_input(_input: &str) -> Self::Parsed {
        todo!("parse your input for day2, **part1**")
    }
}

impl ParseEachInput<Day2, Part2> for AdventOfCode2020 {
    type Parsed = HashMap<String, u32>;

    fn parse_input(_input: &str) -> Self::Parsed {
        todo!("parse your input for day2, **part2**")
    }
}

impl Solution<Day2> for AdventOfCode2020 {
    // different output types are possible too
    type Part1Output = usize;
    type Part2Output = f64;

    fn part1(_part1_input: &Vec<u32>) -> usize {
        // this method gets a Vec
        todo!()
    }

    fn part2(_part2_input: &HashMap<String, u32>) -> f64 {
        // this method gets a HashMap
        todo!()
    }
}

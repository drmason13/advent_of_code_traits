// This example currently uses more awkward code than I would like
// in fact it's easier to implement `ParseInput` than not do so.
//
// But if you want to avoid allocations and have a solution in mind
// that can run using `&str` directly then go for it, it works!

pub struct AdventOfCode2020;
use advent_of_code_traits::{days::Day3, ParseInput, Solution};

fn part1(_input: &str) -> usize {
    todo!("solve day3, part1 without a Typed input")
}

fn part2(_input: &str) -> usize {
    todo!("solve day3, part2 without a Typed input")
}

impl Solution<Day3> for AdventOfCode2020 {
    type Part1Output = usize;
    type Part2Output = usize;

    fn part1(_input: &()) -> usize {
        // you actually need this implementation to appease the compiler
        unimplemented!()
    }

    fn part2(_input: &()) -> usize {
        // you actually need this implementation to appease the compiler
        unimplemented!()
    }

    // we have a minefield of unimplemented!() methods which we will avoid by
    // providing a custom run method...
    fn run(input: &str) {
        // run each part on the input directly
        let part1_output = part1(input);
        let part2_output = part2(input);

        // print the usual output
        println!(
            "Day {0}, Part 1\n\
            {1}\n\n\
            Day {0}, Part 1\n\
            {2}",
            Day3, part1_output, part2_output
        );
    }
}

// Explicitly opt out of parsing the input
impl ParseInput<Day3> for AdventOfCode2020 {
    type Parsed = ();

    // you actually need this implementation to appease the compiler
    fn parse_input(_input: &str) {}
}

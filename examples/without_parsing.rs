// This example currently uses more awkward code than I would like
// but if you want to avoid allocations and have a solution in mind
// that can run using `&str` directly then go for it, it works!

pub struct AdventOfCode2020;
use advent_of_code_traits::{days::Day3, Part1, Part2, Solution};

impl Solution<Day3, Part1> for AdventOfCode2020 {
    type Input = ();
    type Output = usize;

    // don't implement `solve` or `parse` at all, instead implement `run`,
    // overriding the default (which calls solve and parse for you)
    fn run(&self, input: &str) -> anyhow::Result<()> {
        // put your solution here, no parsing needed
        let output = input.len();

        // report has a default implementation you can use to display your soltuion's output
        <Self as Solution<Day3, Part1>>::report(&self, &output)?;
        Ok(())
    }
}

impl Solution<Day3, Part2> for AdventOfCode2020 {
    type Input = ();
    type Output = usize;

    fn run(&self, input: &str) -> anyhow::Result<()> {
        // put your solution here, no parsing needed
        let output = input.len() * 2;

        // call report
        <Self as Solution<Day3, Part2>>::report(&self, &output)?;
        Ok(())
    }
}

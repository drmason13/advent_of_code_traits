use std::{error::Error, fmt::Display};

pub trait Solution<const Day: u8, const Part: u8> {
    type Input;
    type Output: Display;
    type ParseError: Error;

    #[allow(unused_variables)]
    fn parse(&self, input: &str) -> Result<Self::Input, Self::ParseError> {
        panic!(
            "No parsing implemented for Part1, Day{}\n\
            You may want to override the default implementation of the `parse` method that causes this panic",
            Day
        )
    }

    fn solve(&self, input: Self::Input) -> Self::Output;
}

pub trait Runner<const Day: u8, const Part: u8> {
    fn run(&self, input: &str, solution: &impl Solution<Day, Part>) {
        let output = solution.solve(solution.parse(input).expect(&format!(
            "Failed to parse input for day {} part {}",
            Day, Part,
        )));
        self.report(&output);
    }

    fn report(&self, output: &impl Display) {
        println!(
            "Day {} Part {}:\n\
            {}",
            Day, Part, output
        )
    }
}

impl<T, const Day: u8, const Part: u8> Runner<Day, Part> for T where T: Solution<Day, Part> {}

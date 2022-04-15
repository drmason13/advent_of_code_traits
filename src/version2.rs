use std::fmt::Display;

pub trait PartOne<'a, const DAY: u32> {
    type Input;
    type Output: Display;

    fn parse_input(&'a self, input: &'a str) -> Self::Input;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;

    fn run(&'a self, input: &'a str) -> Self::Output {
        let input = <Self as PartOne<DAY>>::parse_input(&self, input);
        <Self as PartOne<DAY>>::solve(&self, &input)
    }
}

pub trait PartTwo<'a, const DAY: u32> {
    type Input;
    type Output: Display;

    fn parse_input(&'a self, input: &'a str) -> Self::Input;

    fn solve(&'a self, input: &Self::Input) -> Self::Output;

    fn run(&'a self, input: &'a str) -> Self::Output {
        let input = <Self as PartTwo<DAY>>::parse_input(&self, input);
        <Self as PartTwo<DAY>>::solve(&self, &input)
    }
}

pub trait Solution<'a, const DAY: u32> {
    fn run(&'a self, raw_input: &'a str);
}

// autoref specialization - call .run() and either this generic implementation or the more efficient SharedSolution implementation will be used.
// TODO: Have this continue to work when more than one Day is implemented, currently we have to do <Year2021 as SharedSolution<'a, Day1>>::run() to disambiguate which day's solution to run which defeats the point of specialization.
// The current workaround is to simply ask users to provide a separate type for each day so there is no ambiguity!
impl<'a, T, const DAY: u32> Solution<'a, DAY> for &'a T
where
    &'a T: PartOne<'a, DAY> + PartTwo<'a, DAY>,
{
    fn run(&'a self, raw_input: &'a str) {
        let parsed_input = <Self as PartOne<DAY>>::parse_input(&self, raw_input);
        let PartOne_output = <Self as PartOne<'a, DAY>>::solve(&self, &parsed_input);

        let parsed_input = <Self as PartTwo<DAY>>::parse_input(&self, raw_input);
        let PartTwo_output = <Self as PartTwo<'a, DAY>>::solve(&self, &parsed_input);

        println!("Day {DAY}\npart 1: {PartOne_output:?}\npart 2: {PartTwo_output:?}");
    }
}

// required for users to impl PartOne for T and get the implementation for &T required for autoref specialization
impl<'a, T, const DAY: u32> PartOne<'a, DAY> for &T
where
    T: PartOne<'a, DAY>,
{
    type Input = <T as PartOne<'a, DAY>>::Input;
    type Output = <T as PartOne<'a, DAY>>::Output;

    fn parse_input(&'a self, raw_input: &'a str) -> Self::Input {
        <T as PartOne<'a, DAY>>::parse_input(&self, raw_input)
    }

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        <T as PartOne<'a, DAY>>::solve(&self, input)
    }
}

// required for users to impl PartTwo for T and get the implementation for &T required for autoref specialization
impl<'a, T, const DAY: u32> PartTwo<'a, DAY> for &T
where
    T: PartTwo<'a, DAY>,
{
    type Input = <T as PartTwo<'a, DAY>>::Input;
    type Output = <T as PartTwo<'a, DAY>>::Output;

    fn parse_input(&'a self, raw_input: &'a str) -> Self::Input {
        <T as PartTwo<'a, DAY>>::parse_input(&self, raw_input)
    }

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        <T as PartTwo<'a, DAY>>::solve(&self, input)
    }
}

pub trait SharedSolution<'a, const DAY: u32>:
    PartOne<'a, DAY> + PartTwo<'a, DAY> + PartOne<'a, DAY, Input = <Self as PartTwo<'a, DAY>>::Input>
{
    fn run(&'a self, raw_input: &'a str) {
        let parsed_input = <Self as PartOne<'a, DAY>>::parse_input(&self, raw_input);
        let PartOne_output = <Self as PartOne<'a, DAY>>::solve(&self, &parsed_input);
        let PartTwo_output = <Self as PartTwo<'a, DAY>>::solve(&self, &parsed_input);

        println!("Day {DAY}\npart 1: {PartOne_output:?}\npart 2: {PartTwo_output:?}");
    }
}

impl<'a, T, const DAY: u32> SharedSolution<'a, DAY> for T where
    T: PartOne<'a, DAY>
        + PartTwo<'a, DAY>
        + PartOne<'a, DAY, Input = <Self as PartTwo<'a, DAY>>::Input>
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use advent_of_code_traits::{days::Day2, Part1, Part2, Solution};

struct AdventOfCode2020;

// this type alias just makes it easier to change our mind what the Input type for both parts should be
type Input = Vec<u32>;
// whatever parsing function you need, but don't copy it, define it once
fn parse(input: &str) -> anyhow::Result<Input> {
    Ok(input
        .chars()
        .map(|c| match c {
            'a'..='f' => 1,
            'g'..='k' => 2,
            'l'..='t' => 3,
            'u'..='z' => 4,
            _ => 0,
        })
        .collect())
}

impl Solution<Day2, Part1> for AdventOfCode2020 {
    type Input = Input;
    type Output = usize;

    fn parse(&self, input: &str) -> anyhow::Result<Self::Input> {
        // reuse the parsing function here
        parse(input)
    }

    fn solve(&self, _input: &Self::Input) -> anyhow::Result<Self::Output> {
        todo!("solve day 1 part 1")
    }
}

impl Solution<Day2, Part2> for AdventOfCode2020 {
    type Input = Input;
    type Output = f64;

    fn parse(&self, input: &str) -> anyhow::Result<Self::Input> {
        // this uses the same parsing function as Part1
        parse(input)
    }

    fn solve(&self, _part2_input: &Self::Input) -> anyhow::Result<Self::Output> {
        todo!("solve day 1 part 2")
    }
}

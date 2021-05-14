use advent_of_code_traits::{days::*, Part1, Part2, Solution};
use anyhow::Context;
use boilerplate::AdventOfCode2020; // swap `boilerplate` with the name of your library crate

// change year here!
const YEAR: u32 = 2020;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);

    let day = args
        .next()
        .expect(
            "need a day to know which solution to run, e.g. `cargo run -- 3 1` to run day 3, part 1",
        )
        .parse()
        .with_context(|| "unable to parse day, just use a number like `3`")?;

    let part = args
        .next()
        .expect(
            "need a part to know which solution to run, e.g. `cargo run -- 3 1` to run day 3, part 1",
        )
        .parse()
        .with_context(|| "unable to parse part, just use a number like `1`")?;

    let input = std::fs::read_to_string(format!("./input/{}/day{}.txt", YEAR, day))
        .with_context(|| format!("no input available for day {}", day))?;

    let aoc = AdventOfCode2020;

    match (day, part) {
        (1, 1) => <AdventOfCode2020 as Solution<Day1, Part1>>::run(&aoc, &input)?,
        (1, 2) => <AdventOfCode2020 as Solution<Day1, Part2>>::run(&aoc, &input)?,
        // add more match arms as you complete solutions

        // adding arms early will fail to compile
        // (2, 1) => <AdventOfCode2020 as Solution<Day2, Part1>>::run(&aoc, &input),
        _ => unimplemented!("no solution available for that day"),
    };
    Ok(())
}

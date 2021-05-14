use advent_of_code_traits::{days::*, Part1, Part2, Solution};
use anyhow::anyhow;

pub struct AdventOfCode2020;
const SOLUTION: AdventOfCode2020 = AdventOfCode2020;

mod day1;

fn main() -> anyhow::Result<()> {
    let day = std::env::args()
        .skip(1)
        .next()
        .expect(
            "need a day to know which solution to run, e.g. `advent_2020 1` to run day 1 solutions",
        )
        .parse()
        .expect("unable to parse day, just use a number like `1`");

    let input = std::fs::read_to_string(&find_input(day)).expect("no input available for that day");

    match day {
        1 => {
            Solution::<Day1, Part1>::run(&SOLUTION, &input)?;
            Solution::<Day1, Part2>::run(&SOLUTION, &input)?;
        }
        _ => {
            return Err(anyhow!(format!(
                "Day {} is not yet implemented, please choose a different day.",
                day
            )))
        }
    }
    Ok(())
}

// you can mostly ignore this, this makes the example run more reliably from different directories
fn find_input(day: u32) -> String {
    let parent_dir = ["examples/cli/input/2020/", "cli/input/2020/", "input/2020/"]
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Please run this example from a folder in or above examples/cli");
    format!("{}/day{}.txt", parent_dir, day)
}

use advent_of_code_traits::{days::*, Solution};

pub struct AdventOfCode2020;

mod day1;

fn call_solution_for_day(day: u32) -> for<'r> fn(&'r str) {
    match day {
        1 => <AdventOfCode2020 as Solution<Day1>>::run,
        // 4 => <AdventOfCode2020 as Solution<Day4>>::run, // fails to compile (until you implement the solution to Day4)
        _ => unimplemented!("no solution available for that day"),
    }
}

fn main() {
    let day = std::env::args()
        .skip(1)
        .next()
        .expect(
            "need a day to know which solution to run, e.g. `advent_2020 1` to run day 1 solutions",
        )
        .parse()
        .expect("unable to parse day, just use a number like `1`");

    let input = std::fs::read_to_string(&find_input(day)).expect("no input available for that day");

    call_solution_for_day(day)(&input)
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

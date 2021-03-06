use advent_of_code_traits::{days::*, run, SolutionRunner};

mod day01;
mod day02;
mod day03;

struct AdventOfCode2021<const DAY: u32>;

fn main() {
    let day = std::env::args()
        .skip(1)
        .next()
        .expect(
            "need a day to know which solution to run, e.g. `cargo run --example cli 1` to run day 1 solutions",
        )
        .parse()
        .expect("unable to parse day, just use a number like `1`");

    let input = std::fs::read_to_string(&find_input(day)).expect("no input available for that day");

    match day {
        // we have to match because the const generic cannot be a runtime value
        1 => run!(AdventOfCode2021::<Day1>, &input),
        2 => run!(AdventOfCode2021::<Day2>, &input),
        3 => run!(AdventOfCode2021::<Day3>, &input),

        // run!(day1::Problem, &input) expands to...
        // {
        //     let problem = day1::Problem;
        //     (&&&problem).run(&input)
        // },

        // the below fails to compile (until you implement the solution to Day4)
        // 2 => run!(day4::Problem, &input),
        x => unimplemented!("no solution available for day {x}"),
    }
}

// you can mostly ignore this, this makes the example run more reliably from different directories
fn find_input(day: u32) -> String {
    let parent_dir = ["examples/cli/input/2021/", "cli/input/2021/", "input/2021/"]
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Please run this example from a folder in or above examples/cli");

    format!("{}/day{:02}.txt", parent_dir, day)
}

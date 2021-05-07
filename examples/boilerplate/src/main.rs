use advent_of_code_traits::{days::*, Solution};
use boilerplate::AdventOfCode2020; // swap `boilerplate` with the name of your library crate

fn call_solution_for_day(day: u32) -> for<'r> fn(&'r str) {
    match day {
        1 => <AdventOfCode2020 as Solution<Day1>>::run,
        // add more match arms as you complete solutions

        // adding arms early will fail to compile
        // 4 => <AdventOfCode2020 as Solution<Day4>>::run,
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

    let input = std::fs::read_to_string(format!("./input/2020/day{}.txt", day))
        // change the year here -------------------------^^^^
        .expect("no input available for that day");

    call_solution_for_day(day)(&input)
}

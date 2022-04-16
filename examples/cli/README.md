# CLI example

An example main function to run your solutions from the command line.

e.g. to run day 1
`cargo run --example cli 1`

3 days are implemented. Check each of them for a different demonstration of implementint the Solution trait:
* `day01.rs` implements Solution with both part1 and part2 implemented and different input types for each
* `day02.rs` implements Solution with both part1 and part2 implemented using a shared input type
* `day03.rs` implements Solution only for part1 and marks part2 as unimplemented using the MissingPartTwo trait

`main.rs` loads input from a file and run the Solution with that input for the day provided as an argument.

Try running the example passing the argument `2`, `3`, or `4`.

You could extend this example by taking the year as an argument as well as the day.

# Examples

[boilerplate](boilerplate/) is a demo crate that shows how everything hangs together.

You can't run this example using cargo, but you can clone this repo and `cd advent_of_code_traits/examples/boilerplate` and then use cargo normally.

All the following examples can be run using cargo: `cargo run --example <example_name>`

### [`cli`](cli/)
`cargo run --example cli`

An example main function to run your solutions from the command line.

### [`parse_each_input`](parse_each_input.rs)
`cargo run --example parse_each_input`

Use `ParseEachInput` to parse a different type for each part of a day.

### [`without_parsing`](without_parsing.rs)
`cargo run --example without_parsing`

Avoid using any sort of parsing entirely and run your solutions against `&str` directly.

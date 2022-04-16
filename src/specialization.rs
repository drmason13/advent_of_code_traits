//! This crate uses autoderef specialization to choose an implementation of [`SolutionRunner`]
//! that is most appropriate to the user's implementation of [`Solution`].
//!
//! There are 3 scenarios I wanted to support without you having to specify anything other than `your_struct.run()`*:
//! 1. You implemented Solution for both Part1 and Part2 and they share the same Input type
//! 2. You implemented Solution for both Part1 and Part2 but they use different Input types
//! 3. You implemented Solution for both Part1 only
//!
//! * for scenario 3, you do have to help the specialization as a user by implementing a marker trait: [`MissingPartTwo`]. We'll talk more about that soon.
//!
//! For a primer on autoderef specialization, I refer you to Lukas' [excellent article](http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html), and of course the [original concept](https://github.com/dtolnay/case-studies/tree/master/autoref-specialization) of autoref specialization from Dtolnay. I'm standing on the shoulders of those standing on the shoulders of giants... and the view up here is wonderful.
//!
//! Where this implementation... struggled at first is because we are exposing so much to the user.
//!
//! We provide a trait that the user has to implement. This is difficult because the author setting up autoderef specialization (me, hello!) is usually the one both writing and implementing the traits. This would leave me free to impl <scenario 1> for some struct behind 2 references, and impl <scenario 3> for some struct behind 0 references and then only expose that struct and tada! it can specialize (or even better, use it internally and never expose it in a public api).
//!
//! But I want you to implement my traits, and I want you to not have to see or deal with the faff of `impl SolutionRunner for &&&YourStruct` because it's gross (I love it, but it's gross).
//!
//! Part of that comes down to providing the [`run!`] macro, which will write `(&&&your_struct).run()` for you.
//!
//! But also it means providing a blanket impl of Solution for &T where T: Solution, this is a double edged-sword.
//! On the one hand it's great because as a user you only need to impl Solution for YourStruct regardless of which specialised method you want to use.
//!
//! On the other hand, as an author it means I have to ensure no overlap in the impls of SolutionRunner I provide.
//! The difficult part of that was stopping scenario 3 (Part1 only) from overlapping with scenario 1 (Part1 and Part2).
//!
//! The trick is to tell the compiler about the **only** part of scenario 3, which is what the [`MissingPartTwo`] trait is for.
//!
//! Another trick used here, which isn't entirely necessary but is quite nice, is to (yet again) use const generics to separate the traits.
//!
//! You know how all the examples of auto(de)ref specialization use separate traits like `ViaString` and `ViaDisplay`?
//! Well one trait with different generic parameters is "different enough" for specialization to work.
//!
//! This is great for this crate because the user is importing these traits, a single trait with a generic parameter is easier to import.
//! It also saves coming up with contrived trait names for each scenario... one of the hard problems in programming sidestepped!
use super::{MissingPartTwo, ParseInput, Solution, SolutionRunner};

// Using auto*de*ref specialization to allow more than 2 specificity levels: http://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
// our autoderef specializations are enumerated as a const parameter to the trait (so the user can import a single trait to run their solutions)
const PART_ONE_ONLY: u16 = 1;
const FULL: u16 = 2;
const SHARED_INPUT_FULL: u16 = 3;

// required for users to be able to simply implement Solution for T and get the implementation for &T required for autoderef specialization
impl<'a, T, const DAY: u32, const PART: u8> Solution<'a, DAY, PART> for &T
where
    T: Solution<'a, DAY, PART>,
{
    type Input = <T as Solution<'a, DAY, PART>>::Input;
    type Output = <T as Solution<'a, DAY, PART>>::Output;

    fn solve(&'a self, input: &Self::Input) -> Self::Output {
        <T as Solution<'a, DAY, PART>>::solve(&self, input)
    }
}

// required for users to be able to simply implement ParseInput for T and get the implementation for &T required for autoderef specialization
impl<'a, T, const DAY: u32, const PART: u8> ParseInput<'a, DAY, PART> for &T
where
    T: ParseInput<'a, DAY, PART>,
{
    type Parsed = <T as ParseInput<'a, DAY, PART>>::Parsed;

    fn parse_input(&'a self, input: &'a str) -> Self::Parsed {
        <T as ParseInput<'a, DAY, PART>>::parse_input(&self, input)
    }
}

// autoderef specialization
impl<'a, T: 'a, const DAY: u32> SolutionRunner<'a, DAY, SHARED_INPUT_FULL> for &'a &'a T
where
    &'a &'a T: Solution<'a, DAY, 1>
        + Solution<'a, DAY, 2>
        + ParseInput<'a, DAY, 1>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + Solution<'a, DAY, 2, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<'a, DAY, 1>>::parse_input(&self, &input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(&self, &parsed_input);
        let part2_output = <Self as Solution<'a, DAY, 2>>::solve(&self, &parsed_input);
        println!("Day {DAY}\npart 1: {part1_output:?}\npart 2: {part2_output:?}");
    }
}

// autoderef specialization
impl<'a, T: 'a, const DAY: u32> SolutionRunner<'a, DAY, FULL> for &'a T
where
    &'a T: Solution<'a, DAY, 1>
        + Solution<'a, DAY, 2>
        + ParseInput<'a, DAY, 1>
        + ParseInput<'a, DAY, 2>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + Solution<'a, DAY, 2, Input = <Self as ParseInput<'a, DAY, 2>>::Parsed>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY, 1>>::parse_input(&self, input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(&self, &parsed_input);

        let parsed_input = <Self as ParseInput<DAY, 2>>::parse_input(&self, input);
        let part2_output = <Self as Solution<'a, DAY, 2>>::solve(&self, &parsed_input);

        println!("Day {DAY}\npart 1: {part1_output:?}\npart 2: {part2_output:?}");
    }
}

// autoderef specialization
impl<'a, T, const DAY: u32> SolutionRunner<'a, DAY, PART_ONE_ONLY> for T
where
    T: Solution<'a, DAY, 1>
        + ParseInput<'a, DAY, 1>
        + Solution<'a, DAY, 1, Input = <Self as ParseInput<'a, DAY, 1>>::Parsed>
        + MissingPartTwo<DAY>,
{
    fn run(&'a self, input: &'a str) {
        let parsed_input = <Self as ParseInput<DAY, 1>>::parse_input(&self, input);
        let part1_output = <Self as Solution<'a, DAY, 1>>::solve(&self, &parsed_input);

        println!("Day {DAY}\npart 1: {part1_output:?}");
    }
}

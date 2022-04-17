# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* A SolutionRunner trait with a run method
* Specialized blanket impls of SolutionRunner for types implementing Solution and ParseInput

### Changed
* BREAKING: Rewrite the Solution trait
    * Add PART generic parameter
    * Swap part1 and part2 methods for a single solve method
    * Swap Part1Output and Part2Output associated types for a single Output associated type
    * Add Input associated type
    * Add &self parameter to solve method
    * Add lifetime parameter
    * Remove run method from Solution trait
* BREAKING: Rewrite and merge the ParseInput and ParseEachInput traits
    * Add PART generic parameter
    * Add &self parameter to parse_input method
    * Add lifetime parameter


## [0.1.1] - 2022-04-17
### Added
* A run! macro to easily run a solution ([@hulufei](https://github.com/hulufei))


## [0.1.0] - 2021-05-08
### Added
* Initial release to crates.io
* traits ParseEachInput, ParseInput, Solution
* consts days::* and Part1, Part2
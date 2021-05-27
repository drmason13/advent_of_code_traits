# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* A run! macro to easily run a solution ([@hulufei](https://github.com/hulufei))

### Changed
* Split run method of Solution into two methods, run and report. By default report prints the output, and run calls report


## [0.1.0] - 2021-05-08
### Added
* Initial release to crates.io
* traits ParseEachInput, ParseInput, Solution
* consts days::* and Part1, Part2
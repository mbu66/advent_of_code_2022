# advent_of_code_2022

This Repo contains my solutions to the [2022 advent of code](https://adventofcode.com/2022). 
My solutions were written in rust on windows and debugging in VScode.

### `src/main.rs`
- includes all the modules of the form day_XX
- calls the `day_XX::run()` for each day
### `src/utils/mod.rs`
- contains useful utility functions used for multiple days
### `src/day_XX/input.txt`
- these files contain the input text from my AOC login
### `src/day_XX/mod.rs`
- contains the solution to the equivalent AOC problem
- includes the utils module
- entry point is `day_XX::run()`
- asserts the correct answer is obtained for both stars

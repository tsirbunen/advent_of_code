## Advent of code 2020 & 2022 with RUST

### Background
In this project, I am exploring the **[rust](https://www.rust-lang.org)** programming language by doing the Advent of code **[2020](https://adventofcode.com/2020)** and **[2022](https://adventofcode.com/2022)** coding challenges. I have no prior knowledge or practice with rust, so pardon my inelegant code.

### Structure
Solutions to the challenges of each year and day have been placed to folders of their own (for example year-2020/day-01, year-2022/day-03, ...). Each folder contains three files:
|File name|Explanation|
| :----------- | :----------- |
|**`solve.rs`**|solution code for both parts 1 and 2|
|**`dev-input.txt`**|simple, small input for development purposes|
|**`prod-input.txt`**|real, larger input for the challenge|


### Running 
To run the code, you must first have rust installed. Then, for some year, day and mode, run in your terminal:
**`cargo run -- <year> <day> <mode>`** 

For example, if you want to run year **2020** and day **1** in mode **dev** , the command is: 
**`cargo run -- 2020 1 dev`**

The available year-day combinations can be found in the table below:
|Year|Days|
| :----------- | :----------- |
|**`2020`**|1 - 2|
|**`2022`**|1|

The available modes are:
- **dev**: simple input of very small size used in developing the solution to the challenge
- **prod**: the real puzzle input (requiring perhaps more processing time)


When the program for the selected year and day runs, it solves both parts 1 and 2, and in the end prints out the solutions for both parts.

### Documentation
To view some documentation, create it with running the following command (if it does not exist)
**`cargo doc`**
Then open file **`target/doc/advent_of_code/index.html`** with a browser.

### Tests
All tests can be run sequentially with the command
**`cargo test -- --test-threads 1`**

If you run the tests with **`cargo test`** (*i.e.* in parallel), it sometimes happens that the tests for `main.rs` (which include running the whole program via command line) fail. 

A single test can be run with command
**`cargo test <test function name>`**

Test organization follows the guidelines of the official documentation: Tests have been placed in the same **_files_** as the functions which they are testing and tests form their own module (even though personally I do not fancy this style).

Tests are mainly unit tests. The framework **[rstest](https://crates.io/crates/rstest/0.17.0)** was selected to help with running same tests with different case scenarios. Note that 100% testing coverage has not been aimed at. 
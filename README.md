# Advent of code

## Running the binary

To run from root of project use:
'''code
cargo run -p day-01 --bin part1
'''

So -p means package. So the day-01 has da Cargo.toml where the name is specified as "day-01"

## Running the test

To run a test from root of project use:
'''code
cargo test -p day-01 --bin part1
'''

So -p means package. So the day-01 has da Cargo.toml where the name is specified as "day-01"

## Create a new day

To create a new day from the root of the project we initialize a new library
'''code
cargo new --lib 2023/day-01
'''

When done, you need to just create a new `bin` folder under the `src` so you would have `src/bin` and move the `lib.rs` to that `bin` folder. You can then rename the file to `part1.rs`

_Do not forget_
To be able to run or test the files you need to have the days in

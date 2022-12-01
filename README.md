# Advent of Code 2022

Rust solves for Advent of Code 2022.

Each day has it's own source file with builtin tests and it's own input file.

Some days are broken up between parts 1 and 2 completely, rereading input and such, but not all day.

Hopefully this will help somebody.

## Usage
Feel free to use as you see fit.

```
❯ cargo run --release
   Compiling aoc22 v0.1.0 (/home/icon/Projects/aoc22)
    Finished release [optimized] target(s) in 0.21s
     Running `target/release/aoc22`
Running All Day Solves:
        Day 1 Part 1: 68442
        Day 1 Part 2: 204837
Day 1: 112.95µs 

❯ cargo test
   Compiling aoc22 v0.1.0 (/home/icon/Projects/aoc22)
    Finished test [unoptimized + debuginfo] target(s) in 0.17s
     Running unittests src/main.rs (target/debug/deps/aoc22-007f50c710f9889e)

running 1 test
test day1::tests::day1_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
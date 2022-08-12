# advent_of_code_2021

Solutions for Advent of Code 2021 written in rust.

Some solutions are a bit scuffed, but the overall time to run both part 1 and part 2 for all days takes ~2.3 seconds, which I'm happy with.

Run with:
  > cargo run --release 

To run a specific day:
  > cargo run --release {day_number}
 
To run with example input:
  > cargo run --release {day_number} --example


Note:
  - Some days require to be run with --release flag, since they depend on number overflowing, which raises error in development mode.
  - Day 13 part 2 output isn't the real solution; uncomment the result printout which will display human readable number, which is the actual solution.

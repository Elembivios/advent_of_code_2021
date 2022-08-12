# advent_of_code_2021

Solutions for Advent of Code 2021 written in rust.
Some solutions are a bit scuffy, but the overall time to run both part 1 and part 2 for all days takes ~2.3 seconds, which I'm happy with. 
Run with --release flag:
  > cargo run --release 
 .. since some days depend on number overflowing, which raises error in development mode.
 
 To run a specific day:
  > cargo run --release {day_number}
 
 To run with example input:
  > cargo run --release {day_number} --example
 

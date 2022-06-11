mod euclidean;
// mod data_structures;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;

use std::{error::Error, fs};
use structopt::StructOpt;
use owo_colors::OwoColorize;
use std::time::{Duration, Instant};
use anyhow::Context;
use humantime::format_duration;


trait Advent {
    fn new(data: &str) -> Self
    where 
        Self: Sized;
    fn part1(&mut self) -> usize;
    fn part2(&mut self) -> usize;    
}

struct Solution {
    event: Box<dyn Advent>,
    time: Duration,
}

impl Solution {
    fn new<Event: Advent + 'static>(content: &str) -> Self {
        let (event, time) = get_time(||Event::new(content));

        Solution {
            event: Box::new(event),
            time,
        }
    }

    fn get_result(&mut self, day: u32) {
        let (part1, time1) = get_time(|| self.event.part1());
        let (part2, time2) = get_time(|| self.event.part2());

        println!("Solution for day {}", day);
        println!(
            "Collect data in {}",
            format_duration(self.time).fg_rgb::<255, 63, 128>()
        );
        println!(
            "Part 1: {} in {}",
            part1.fg_rgb::<100,252,218>(),
            format_duration(time1).fg_rgb::<100, 252,218>()
        );
        println!(
            "Part 2: {} in {}",
            part2.fg_rgb::<100, 252, 218>(),
            format_duration(time2).fg_rgb::<100, 252, 218>()
        );
    }
}

#[derive(StructOpt)]
struct Cli {
    day: u32,

    #[structopt(short, long, help= "Uses example file provided by AOC")]
    example: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let main_file = if args.example { "example" } else { "input" };
    let filename = format!("src/day_{:02}/{}.txt", args.day, main_file);

    let mut content: &str = &fs::read_to_string(filename)
        .with_context(|| format!("Could not read {} file for day {}", main_file, args.day))?;
    content = content.trim();

    let mut solution = match args.day {
        1 => Solution::new::<day_01::SonarSweep>(content),
        2 => Solution::new::<day_02::Dive>(content),
        3 => Solution::new::<day_03::BinaryDiagnostic>(content),
        4 => Solution::new::<day_04::GiantSquid>(content),
        5 => Solution::new::<day_05::HydrothermalVenture>(content),
        6 => Solution::new::<day_06::Lanternfish>(content),
        7 => Solution::new::<day_07::TheThreacheryOfWhales>(content),
        8 => Solution::new::<day_08::SevenSegmentSearch>(content),
        9 => Solution::new::<day_09::SmokeBasin>(content),
        10 => Solution::new::<day_10::SyntaxScoring>(content),
        11 => Solution::new::<day_11::DumboOctopus>(content),
        12 => Solution::new::<day_12::PassagePassing>(content),
        13 => Solution::new::<day_13::TransparentOrigami>(content),
        14 => Solution::new::<day_14::ExtendedPolymerization>(content),
        15 => Solution::new::<day_15::Chiton>(content),
        16 => Solution::new::<day_16::PacketDecoder>(content),
        17 => Solution::new::<day_17::TrickShot>(content),
        18 => Solution::new::<day_18::Snailfish>(content),
        _ => unreachable!(),
    };

    solution.get_result(args.day);

    Ok(())    
}

fn get_time<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();

    (result, time)
}



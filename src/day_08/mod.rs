pub struct SevenSegmentSearch {
    let signal_patterns: Vec<Display>,
    let output: Vec<Display>,
}

pub struct Display {
    let pattern: &str,
}

impl crate::Advent for SevenSegmentSearch {
    fn new(data: &str) -> SevenSegmentSearch {
        data
            .lines()
            .map(|&s| {
                let iter = s.split(" | ");
                let signal_patterns = iter.next().unwrap().split(' ').map(|s| Display { pattern: s }).collect();
                let output = iter.next().unwrap().split(' ').map(|s| Display { pattern: s }).collect();
                (signal_patterns, output)
            })
            .collect()

    }

    fn part1(&self) -> usize {
        2
    }

    fn part2(&self) -> usize {
        3
    }
}
use std::str::FromStr;

enum Direction {
    Forward,
    Down,
    Up
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }        
    }
}

struct Command {
    direction: Direction,
    count: usize,
}

pub struct Dive {
    commands: Vec<Command>,
}

impl crate::Advent for Dive {
    fn new(data: &str) -> Dive {
        let commands: Vec<Command> = data 
            .lines()
            .map(|d| {
                let mut cmd_iter = d.split_whitespace();
                let direction = Direction::from_str(cmd_iter.next().unwrap()).unwrap();
                let count = cmd_iter.next().unwrap().parse::<usize>().unwrap();

                Command { direction, count }
            })
            .collect();
        Dive { commands }
    }

    fn part1(&mut self) -> usize {
        let mut horizontal: usize = 0;
        let mut depth: usize = 0;

        self.commands.iter().for_each(|c| match c.direction {
            Direction::Forward => horizontal += c.count,
            Direction::Down => depth += c.count,
            Direction::Up => depth -= c.count,
        });

        (horizontal * depth) as usize
    }

    fn part2(&mut self) -> usize {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        self.commands.iter().for_each(|c| match c.direction {
            Direction::Forward => {
                horizontal += c.count;
                depth += aim * c.count;
            },
            Direction::Down => aim += c.count,
            Direction::Up => aim -= c.count,
        });

        (horizontal * depth) as usize
    }
}


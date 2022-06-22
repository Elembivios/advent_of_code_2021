use std::fmt;
use super::euclidean::{Coordinate, Axis};

pub struct TransparentOrigami {
    coordinates: Vec<Coordinate<u32>>,
    folds: Vec<Fold>,
    num_folded: usize,
    width: u32,
    height: u32
}

impl TransparentOrigami {    
    fn fold(&mut self) -> () {
        let fold = &self.folds[self.num_folded];
        self.coordinates.iter_mut().filter(|c| {
            match fold.axis {
                Axis::X => {c.x > fold.value},
                Axis::Y => {c.y > fold.value}
            }
        }).for_each(|c| {
            match fold.axis {
                Axis::X => { c.x = c.x - 2 * (c.x - fold.value); },
                Axis::Y => { c.y = c.y - 2 * (c.y - fold.value); }
            }
        });

        self.coordinates.sort();
        self.coordinates.dedup();
        self.num_folded += 1;
        match fold.axis {
            Axis::X => self.width = fold.value,
            Axis::Y => self.height = fold.value
        }
    }
}

impl fmt::Display for TransparentOrigami {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "TransparentOrigami ({} x {})\n", self.width, self.height)?;
        let line_numbers: String = (0..self.width).fold(
            String::new(), 
            |acc, num: u32| {acc + format!("{: >2}", &num).as_str()}
        );
        write!(f, "\t X {}\n", line_numbers)?;
        for y in 0..self.height {
            let mut line: String = String::new();
            for x in 0..self.width {
                let coordinate = self.coordinates.iter().find(|c| c.x == x && c.y == y);
                if let Some(_) = coordinate {
                    line.push('⬛');
                } else {
                    line.push('⬜');
                }
            }
            write!(f, "\t{: >2} {}\n", y, line)?;
        }
        write!(f, "\n")
    }
}

struct Fold {
    axis: Axis,
    value: u32
}

impl crate::Advent for TransparentOrigami {
    fn new(data: &str) -> TransparentOrigami {
        let (coordinates_data, folds_data) = data.split_once("\r\n\r\n").unwrap_or_else(
            || {data.split_once("\n\n").unwrap()}
        );
        let coordinates: Vec<Coordinate<u32>> = coordinates_data
            .lines()
            .map(|c| {
                let (x, y) = c.split_once(",").unwrap();
                Coordinate { x: x.parse().unwrap(), y: y.parse().unwrap() }
            }).collect();

        let folds: Vec<Fold> = folds_data
            .lines()
            .map(|line| {
                let (text, value) = line.split_once("=").unwrap();
                let axis = match text.chars().last().unwrap() {
                    'x' => Axis::X,
                    'y' => Axis::Y,
                    axis_label => panic!("Invalid axis label: {}", axis_label)
                };
                Fold { axis, value: value.parse().unwrap() }
            }).collect();
        let max_x = coordinates.iter().map(|c| c.x).max().unwrap();
        let max_y = coordinates.iter().map(|c| c.y).max().unwrap();
        TransparentOrigami { coordinates, folds, num_folded: 0, width: max_x + 1, height: max_y + 1 }
    }

    fn part1(&mut self) -> usize {
        self.fold();
        self.coordinates.len()
    }
    
    fn part2(&mut self) -> usize {
        let remaining_folds = self.folds.len() - 2;
        for _ in 0..=remaining_folds {
            self.fold();
        }

        // This is the actual solution -- displayed as colored squares
        // println!("Result: {}", self);

        self.coordinates.len()
    }
}
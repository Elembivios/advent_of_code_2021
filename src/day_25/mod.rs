use crate::euclidean::{Coordinate, Axis};
use std::fmt;

#[derive(Clone, Debug)]
struct Cucumber {
    pos: Coordinate<u8>,
    dir: Axis
}

#[derive(Clone)]
struct Map {
    cucumbers: Vec<Vec<Option<Cucumber>>>,
    width: u8,
    height: u8
}

#[allow(dead_code)]
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for row in &self.cucumbers {
            let row_str: String = row.iter().map(|space| {
                match space {
                    Some(cucumber) => {
                        match cucumber.dir {
                            Axis::X => '>',
                            Axis::Y => 'v'
                        }
                    },
                    None => { '.' }
                }
            }).collect();

            write!(f, "{}\n", row_str)?;
        }

        write!(f, "\n")
    }
}

impl Map {
    fn new(cucumbers: Vec<Vec<Option<Cucumber>>>) -> Self {
        let first = &cucumbers[0];
        let height = cucumbers.len() as u8;
        let width = first.len() as u8;
        Map { cucumbers, width, height }
    }
    fn next_position(&self, cucumber: &Cucumber) -> Coordinate<u8> {
        match cucumber.dir {
            Axis::X => {
                let mut x = cucumber.pos.x + 1;
                if x >= self.width {
                    x = 0;
                }
                Coordinate {
                    x,
                    y: cucumber.pos.y
                }
            }
            Axis::Y => {
                let mut y = cucumber.pos.y + 1;
                if y >= self.height {
                    y = 0;
                }
                Coordinate {
                    x: cucumber.pos.x,
                    y
                }
            }
        }
    }
    fn get_space(&self, pos: &Coordinate<u8>) -> &Option<Cucumber> {
        &self.cucumbers[pos.y as usize][pos.x as usize]
    }
}

pub struct SeaCucumber {
    map: Map
}

impl crate::Advent for SeaCucumber {
    fn new(data: &str) -> SeaCucumber {
        let cucumbers = data.lines().enumerate().map(|(y, l)| {
            l.chars().enumerate().map(|(x, c)| {
                match c {
                    '.' => None,
                    '>' => Some(Cucumber { 
                        pos: Coordinate { x: x as u8, y: y as u8},
                        dir: Axis::X,
                    }),
                    'v' => Some(Cucumber {
                        pos: Coordinate { x: x as u8, y: y as u8 },
                        dir: Axis::Y
                    }),
                    _ => unreachable!()
                }
            }).collect()          
        }).collect();
        let map = Map::new(cucumbers);
        SeaCucumber { map }
    }

    fn part1(&mut self) -> usize {
        let mut map = self.map.clone();
        let mut num_moved =  usize::MAX;
        let mut iteration = 0;

        while num_moved != 0 {            
            num_moved = 0;
            let east_swaps: Vec<Vec<(Coordinate<u8>, Coordinate<u8>)>> = map.cucumbers
                .iter()
                .map(|row| {
                    // Get east facing cucumbers
                    row.iter().filter_map(|space| {
                        match space {
                            Some(cucumber) => {
                                match cucumber.dir {
                                    Axis::X => Some(cucumber),
                                    Axis::Y => None
                                }
                            }, 
                            None => None
                        }
                    // Get swaps if next position is clear
                    }).filter_map(|cucumber| {
                        let next_pos = map.next_position(cucumber);
                        let next_space = map.get_space(&next_pos);
                        if next_space.is_none() {
                            Some((cucumber.pos.clone(), next_pos))
                        } else {
                            None
                        }
                    }).collect()                  
                }).collect();        
            // Move east facing cucumbers
            map.cucumbers.iter_mut().zip(east_swaps).for_each(|(row, row_swaps)| {
                for swap in row_swaps {
                    // row.swap(swap.0.x as usize, swap.1.x as usize);
                    let mut source = std::mem::replace(&mut row[swap.0.x as usize], None);
                    if let Some(ref mut cucumber) = source {
                        cucumber.pos = swap.1.clone();
                    }
                    let destination = std::mem::replace(&mut row[swap.1.x as usize], source);
                    row[swap.0.x as usize] = destination;                    
                    num_moved += 1;
                }
            });

            let south_swaps: Vec<(Coordinate<u8>, Coordinate<u8>)> = map.cucumbers
                .iter()
                .flatten()
                // Get south facing cucumbers
                .filter_map(|space| {
                    match space {
                        Some(cucumber) => {
                            match cucumber.dir {
                                Axis::X => None,
                                Axis::Y => Some(cucumber)
                            }
                        },
                        None => None
                    }
                // Get swaps if next space is clear
                }).filter_map(|cucumber| {
                    let next_pos = map.next_position(cucumber);
                    let next_space = map.get_space(&next_pos);
                    if next_space.is_none() {
                        Some((cucumber.pos.clone(), next_pos))
                    } else {
                        None
                    }
                }).collect();

            num_moved += south_swaps.len();
            for swap in south_swaps {
                let mut source = std::mem::replace(&mut map.cucumbers[swap.0.y as usize][swap.0.x as usize], None);
                if let Some(ref mut cucumber) = source {
                    cucumber.pos = swap.1.clone();
                }
                let destination = std::mem::replace(&mut map.cucumbers[swap.1.y as usize][swap.1.x as usize], source);
                map.cucumbers[swap.0.y as usize][swap.0.x as usize] = destination;
            }
            iteration += 1;
        }
        iteration
    }

    fn part2(&mut self) -> usize {
        2
    }
}
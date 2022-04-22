// use owo_colors::OwoColorize;
// use std::io::stdin; 
const WIDTH: usize = 10;

pub struct DumboOctopus {
    octopuses: [[u8; WIDTH]; WIDTH],
}

const ADJACENT_OFFSET: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

fn add(u: usize, i: isize) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn in_bounds(x: usize, y: usize) -> bool {
    x < WIDTH && y < WIDTH
}

impl DumboOctopus {    
    fn neighbours(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();
        for (ox, oy) in ADJACENT_OFFSET {
            let ny = add(y, oy);
            let nx = add(x, ox);
            if let (Some(nx), Some(ny)) = (nx, ny) {
                if in_bounds(nx, ny) {
                    neighbours.push((nx, ny));
                }
            }
        }
        neighbours
    }

    fn pass_cycle(&mut self) -> usize {
        self.octopuses.iter_mut().flatten().for_each(|v| *v += 1);

        let mut flag: bool = true;
        let mut num_flashes: usize = 0;
        while flag {
            flag = false;
            for y in 0..self.octopuses.len() {
                for x in 0..self.octopuses[0].len() {
                    if self.octopuses[y][x] > 9 {
                        // Flash
                        flag = true;                        
                        num_flashes += 1;
                        self.octopuses[y][x] = 0;
                        let neighbours = self.neighbours(x, y);
                        neighbours
                            .into_iter()
                            .for_each(|(nx, ny)| {
                                if self.octopuses[ny][nx] != 0 {
                                    self.octopuses[ny][nx] += 1;
                                }                                
                        });
                    }
                }
            }
        }        
        num_flashes
    }

    // fn display(&self, label: &str) -> () {
    //     println!("{}", label);        
    //     self.octopuses.iter().for_each(|row| {
    //         row.iter().for_each(|val| {
    //             if *val >= 9 {
    //                 print!("{:<2} ", val.red());
    //             } else if *val == 0 {
    //                 print!("{:<2} ", val.green());
    //             } else {
    //                 print!("{:<2} ", val);
    //             }               
    //         });
    //         println!();
    //     });        
    //     println!("-----------------------");
    // }
}

impl crate::Advent for DumboOctopus {
    fn new(data: &str) -> DumboOctopus {
        let octopuses: [[u8; WIDTH]; WIDTH] = data.lines().map(|l| {
            l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>().try_into().unwrap()
        }).collect::<Vec<[u8; WIDTH]>>().try_into().unwrap();
        DumboOctopus { octopuses }
    }

    fn part1(&mut self) -> usize {
        let mut num_flashes: usize = 0;
        for _ in 0..100 {
            num_flashes += self.pass_cycle();
        }        
        num_flashes
    }

    fn part2(&mut self) -> usize {
        let mut passed_cycles: usize = 100;
        let size = WIDTH * WIDTH;
        while self.pass_cycle() != size {
            passed_cycles += 1;
        }
        passed_cycles + 1
    }
}

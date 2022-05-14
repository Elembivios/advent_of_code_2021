use super::euclidean::{Point, Coordinate};

// Aliases
type P = Point<u8, usize>;
type Line = Vec<P>;
type Map = Vec<Line>;

pub struct Chiton {
    map_p1: Grid,
    map_p2: Grid
}

struct Grid {
    map: Map,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(map: Map) -> Grid {
        let height: usize = map.len();
        let width = match height {
            0 => 0,
            _ => map[0].len()
        };
        Grid { map, height, width }
    }

    fn neighbors(&self, point: &P) -> Vec<&P> {
        let mut neighbors: Vec<&P> = vec![];
        if point.coordinate.x > 0 {
            neighbors.push(self.get_point(point.coordinate.x - 1, point.coordinate.y));        
        }

        if point.coordinate.x < self.width - 1 {
            neighbors.push(self.get_point(point.coordinate.x + 1, point.coordinate.y));            
        }

        if point.coordinate.y > 0 {
            neighbors.push(self.get_point(point.coordinate.x, point.coordinate.y - 1));            
        }

        if point.coordinate.y < self.height - 1 {
            neighbors.push(self.get_point(point.coordinate.x, point.coordinate.y + 1));
        } 

        neighbors
    }

    fn get_point(&self, x: usize, y: usize) -> &P {
        &self.map[y][x]
    }
}

fn copy_grid(grid: &Grid, n: usize) -> Grid{
    let mut new_map : Map = vec![];

    for y in 0..n {        
        for (gy, line) in grid.map.iter().enumerate() {
            let mut new_line: Vec<P> = vec![];
            for x in 0..n {
                for (gx, p) in line.iter().enumerate() {
                    let mut new_val = p.val + x as u8 + y as u8;
                    if new_val > 9 {
                        new_val = new_val % 9;
                    }
                    let new_point = Point {
                        val: new_val,
                        coordinate: Coordinate {
                            x: p.coordinate.x + (x * grid.width),
                            y: p.coordinate.y + (y * grid.height)
                        }
                    };
                    new_line.push(new_point);
                }
            }
            new_map.push(new_line);
        }
    }
    Grid::new( new_map )
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Chiton {
        let data: Map = data.lines().enumerate().map(|(y, line)| {
            let points: Line = line.chars().enumerate().map(|(x, c)| {
                let val: u8 = c.to_digit(10).unwrap() as u8;
                let coordinate: Coordinate<usize> = Coordinate { x, y };
                Point { val, coordinate }
            }).collect();
            points
        }).collect();
        let part1 = Grid::new(data);
        let part2 = copy_grid(&part1, 5);
        Chiton {map_p1: part1, map_p2: part2}
    }

    fn part1(&mut self) -> usize {
        let start = self.map_p1.get_point(0, 0);
        let goal = self.map_p1.get_point(self.map_p1.width - 1, self.map_p1.height - 1);
        let heuristic = |current: &P, goal: &P| {
            let x = goal.coordinate.x - current.coordinate.x;
            let y = goal.coordinate.y - current.coordinate.y;
            let future_path = x + y;
            future_path
        };
        let path = a_star(&self.map_p1, start, goal, heuristic);

        let cost = match path {
            Some(path) => {
                path.iter().map(|p| {
                    p.val as usize
                }).sum()
            },
            None => 0
        };

        cost - start.val as usize
    }

    fn part2(&mut self) -> usize {
        let start = self.map_p2.get_point(0, 0);
        let goal = self.map_p2.get_point(self.map_p2.width - 1, self.map_p2.height - 1);
        let heuristic = |current: &P, goal: &P| {
            let x = goal.coordinate.x - current.coordinate.x;
            let y = goal.coordinate.y - current.coordinate.y;
            let future_path = x + y;
            future_path
        };
        let path = a_star(&self.map_p2, start, goal, heuristic);

        let cost = match path {
            Some(path) => {
                path.iter().map(|p| {
                    p.val as usize
                }).sum()
            },
            None => 0
        };

        cost - start.val as usize
    }
}

use std::collections::HashMap;

fn reconstuct_path<'a>(came_from:  &HashMap<&'a P, &'a P>, current: &'a P) -> Vec<&'a P> {    
    let mut total_path: Vec<&P> = vec![current];        
    let mut previous_point = came_from.get(current);    
    while previous_point.is_some() {
        if let Some(p) = previous_point {
            total_path.insert(0, p.clone());
            previous_point = came_from.get(p);
        }
    }
    total_path
}

fn d(_current: &P, neighbor: &P) -> usize {
    neighbor.val as usize
}
fn a_star<'a>(grid: &'a Grid, start: &'a P, goal: &'a P, heuristic: fn(&P, &P) -> usize) -> Option<Vec<&'a P>> {

    let mut open_set = vec![start];

    let mut came_from: HashMap<&'a P, &'a P> = HashMap::new();

    let mut g_score: HashMap<&P, usize> = grid.map.iter().flatten().map(|p| (p, usize::MAX)).collect();
    g_score.insert(start, 0);
    
    let mut f_score: HashMap<&P, usize> = grid.map.iter().flatten().map(|p| (p, usize::MAX)).collect();
    f_score.insert(start, heuristic(&start, goal));

    while open_set.len() != 0 {
        let min_f = open_set.iter().map(|p| {
            (p, f_score.get(p).unwrap())
        }).min_by(|a, b| {
            a.1.cmp(b.1)
        }).unwrap();
        let current = *min_f.0;

        if current.coordinate == goal.coordinate { // Comparing references?
            return Some(reconstuct_path(&came_from, current))
        }

        // display(grid, &open_set, current, &reconstuct_path(&came_from, current));

        let pos = open_set.iter().position(|p| *p == current).unwrap();
        open_set.remove(pos);

        for neighbor in grid.neighbors(current) {        
            let tentative_g_score = g_score[current] + d(current, neighbor);
            
            if tentative_g_score < g_score[neighbor] {
                came_from.insert(neighbor, current);
                let tentative_f_score = tentative_g_score + heuristic(neighbor, goal);
                g_score.insert(neighbor, tentative_g_score);            
                f_score.insert(neighbor, tentative_f_score);
                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }                 
        }        
    }
    None
}

use owo_colors::OwoColorize;
use std::io;

fn input() -> () {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();    
}

fn display(grid: &Grid, open_set: &Vec<&P>, current: &P, path: &Vec<&P>) -> () {
    for line in &grid.map {        
        for p in line {
            if p == current {
                print!("{:<2}", p.val.red());
            }
            else if path.contains(&p) {
                print!("{:<2}", p.val.green());
            }
            else if open_set.contains(&p) {
                print!("{:<2}", p.val.bright_white());
            } else {
                print!("{:<2}", p.val.fg_rgb::<100, 252,218>())
            }            
        }
        print!("\n");
    }
    println!("---------------------");
    input();
}
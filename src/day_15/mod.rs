use super::euclidean::{Point, Axis, Coordinate};

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
        let width: usize = map[0].len();
        Grid { map, height, width }
    }

    fn neighbors(&self, point: &P) -> Vec<&P> {
        let neighbors: Vec<&P> = vec![];
        if point.coordinate.x > 0 {
            neighbors.push(self.get_point(point.coordinate.x - 1, point.coordinate.y));        
        }

        if point.coordinate.x < self.width {
            neighbors.push(self.get_point(point.coordinate.x + 1, point.coordinate.y));            
        }

        if point.coordinate.y > 0 {
            neighbors.push(self.get_point(point.coordinate.x, point.coordinate.y - 1));            
        }

        if point.coordinate.y < self.height {
            neighbors.push(self.get_point(point.coordinate.x, point.coordinate.y + 1));
        } 

        neighbors
    }

    fn get_point(&self, x: usize, y: usize) -> &P {
        &self.map[y][x]
    }
}

impl Chiton {
    fn neighbors(&self, point: P) -> Vec<&P>{
        let neighbors: Vec<&P> = vec![];


        neighbors
    }
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Chiton {
        let data: Map = data.lines().enumerate().map(|(x, line)| {
            let points: Line = line.chars().enumerate().map(|(y, c)| {
                let val: u8 = c.to_digit(10).unwrap() as u8;
                let coordinate: Coordinate<usize> = Coordinate { x, y };
                Point { val, coordinate }
            }).collect();
            points
        }).collect();
        let part1 = Grid::new(data);
        let part2 = Grid::new(vec![]);
        Chiton {map_p1: part1, map_p2: part2}
    }

    fn part1(&mut self) -> usize {


        1
    }

    fn part2(&mut self) -> usize {
        2
    }
}

use std::collections::HashMap;

fn reconstuct_path(came_from: HashMap<&P, &P>, current: &P) -> Vec<P> {    
    let mut total_path = vec![];
    let mut current_point = Some(current);

    // A bit different implementation
    while current_point.is_some() {
        total_path.insert(0, *current_point.unwrap().clone());
        let current_point = came_from.remove(current_point.unwrap());
    }
    total_path
}

fn d(current: &P, neighbor: &P) -> usize {
    3
}

fn a_star(grid: Grid, start: &P, goal: &P, h: fn(&P) -> usize) -> Option<Vec<P>> {
    let mut open_set = vec![start];

    let came_from: HashMap<&P, &P> = HashMap::new();

    // TODO: Default value of infinity
    let g_score: HashMap<&P, usize> = HashMap::from([
        (start, 0)
    ]);
    
    // TODO: Default value of infinity
    let f_score: HashMap<&P, usize> = HashMap::from([
        (start, h(&start))
    ]);

    while open_set.len() != 0 {
        let current = *open_set.iter().min_by(|a, b| {
            a.coordinate.cmp(&b.coordinate)
        }).unwrap();

        if current == goal { // Comapring references?
            return Some(reconstuct_path(came_from, current))
        }

        let pos = open_set.iter().position(|p| *p == current).unwrap();
        open_set.remove(pos);

        for neighbor in grid.neighbors(current) {
            let tentative_g_score = g_score[current] + d(current, neighbor);
            g_score[neighbor] = tentative_g_score;
            f_score[neighbor] = tentative_g_score + h(neighbor);
            if !open_set.contains(&neighbor) {
                open_set.push(neighbor);
            }
        }                
    }
    None
}
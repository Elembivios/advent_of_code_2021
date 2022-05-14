use super::euclidean::{Point, Coordinate};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::iter::once;

// Aliases 
type P = Point<u8, usize>;
type Grid = Vec<Vec<State>>;

pub struct Chiton {
    map_p1: Map,
    map_p2: Map
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Chiton {
        let data: Grid = data.lines().enumerate().map(|(y, line)| {
            let states: Vec<State> = line.chars().enumerate().map(|(x, c)| {
                let val: u8 = c.to_digit(10).unwrap() as u8;
                let coordinate: Coordinate<usize> = Coordinate { x, y };
                State::new(Point { val, coordinate })
            }).collect();

            states
        }).collect();
        let part1 = Map::new(data);
        let part2 = part1.copy_data(5);
        Chiton { map_p1: part1, map_p2: part2 }
    }

    fn part1(&mut self) -> usize {
        let map = &self.map_p1;

        let start = map.get_state(0, 0);
        let goal = map.get_state(map.width - 1, map.height - 1);

        let path = map.a_star(start, goal);

        1

    }

    fn part2(&mut self) -> usize {
        2
    }


}
#[derive(Eq, PartialEq, Hash)]
struct State {
    p: P,
    f_score: usize,
    g_score: usize,
}

impl State {
    fn new(p: P) -> State {
        State { p, f_score: usize::MAX, g_score: usize::MAX }
    }

    fn heuristic(&self, goal: &Self) -> usize {
        let x = goal.p.coordinate.x - self.p.coordinate.x;
        let y = goal.p.coordinate.y - self.p.coordinate.y;
        x + y
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
        .then_with(|| self.p.coordinate.cmp(&other.p.coordinate))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    data: Grid,
    height: usize,
    width: usize
}

impl Map {
    fn new(data: Grid) -> Map {
        let height = data.len();
        let width = match height {
            0 => 0,
            _ => data[0].len()
        };
        Map { data, height, width }
    }

    fn copy_data(&self, n: usize) -> Map {
        let mut new_data: Grid = vec![];

        for y in 0..n {
            for (gy, line) in self.data.iter().enumerate() {
                let mut new_line: Vec<State> = vec![];
                for x in 0..n {
                    for (gx, state) in line.iter().enumerate() {
                        let mut new_val = state.p.val + x as u8 + y as u8;
                        if new_val > 9 { new_val = new_val % 9 };
                        let new_point = Point {
                            val: new_val,
                            coordinate: Coordinate {
                                x: state.p.coordinate.x + (x * self.width),
                                y: state.p.coordinate.y + (y * self.height)
                            }
                        };
                        new_line.push(State::new( new_point ));
                    }
                }
                new_data.push(new_line)
            }
        }

        Map::new(new_data)
    }

    fn neighbors(&self, c: &Coordinate<usize>) -> Vec<&State> {
        let mut neighbors: Vec<&State> = vec![];
        if c.x > 0 { neighbors.push(self.get_state( c.x - 1, c.y )); }
        if c.x < self.width - 1 { neighbors.push(self.get_state( c.x + 1, c.y )); }
        if c.y > 0 { neighbors.push(&mut self.get_state( c.x, c.y - 1)); }
        if c.y < self.height - 1 { neighbors.push(&mut self.get_state( c.x, c.y + 1)); }
        neighbors
    }

    fn neighbors_iter_mut(&mut self, c: &Coordinate<usize>) -> NeighborsIterMut {
        NeighborsIterMut::new(self, c.x, c.y)
    }

    fn get_state_mut(&mut self, x: usize, y: usize) -> &mut State {
        &mut self.data[y][x]
    }

    fn get_state(&self, x: usize, y: usize) -> &State {
        &self.data[y][x]
    }


    fn a_star(&mut self, start: &State, goal: &State) -> Option<Vec<&P>> {
        let mut open_set = BinaryHeap::new();
        open_set.push(start);

        let mut came_from: HashMap<&P, &P> = HashMap::new();

        while let Some(state) = open_set.pop() {
            if state == goal {
                return Some(self.reconstruct_path(&came_from, &state.p));
            }


            for neighbor in self.neighbors_iter_mut(&state.p.coordinate) {
                let tentative_g_score = state.g_score + neighbor.p.val as usize;
                if tentative_g_score < neighbor.g_score {
                    came_from.insert(&neighbor.p, &state.p);
                    let tentative_f_score = tentative_g_score + neighbor.heuristic(goal);
                    neighbor.g_score = tentative_g_score;
                    neighbor.f_score = tentative_f_score;
                    if let None = open_set.iter().find(|s| s.p.coordinate == neighbor.p.coordinate) {
                        open_set.push(&neighbor);
                    }
                }
            }
        }
        None
    }

    fn reconstruct_path<'a>(&self, came_from: &HashMap<&'a P, &'a P>, current: &'a P) -> Vec<&'a P> {
        let mut total_path: Vec<&P> = vec![&current.p];
        let mut previous_point = came_from.get(current.p);


    }
}

enum Operation {
    Sub,
    Add,
    None
}
struct NeighborsIterMut<'a> {
    map: &'a mut Map,
    x: usize,
    y: usize,
    index: usize,
    offsets: [[Operation;2]; 4]
}

impl<'a> NeighborsIterMut<'a> {
    fn new(map: &'a mut Map, x: usize, y: usize) -> NeighborsIterMut<'a> {
        NeighborsIterMut{ 
            map, x, y, index: 0, offsets: [
                [Operation::Sub, Operation::None],
                [Operation::None, Operation::Sub],
                [Operation::Add, Operation::None],
                [Operation::None, Operation::Add]
            ]
        }
    }
}
impl<'a> Iterator for NeighborsIterMut<'a> {
    type Item = &'a mut State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.offsets.len() - 1 {

            let operation = &self.offsets[self.index];
            let x = match operation[0] {
                Operation::None => Some(self.x),
                Operation::Add => { if self.x < self.map.width - 1 { Some(self.x + 1) } else { None }},
                Operation::Sub => { if self.x > 0 { Some(self.x - 1) } else { None }}
            };
            let y = match operation[1] {
                Operation::None => Some(self.y),
                Operation::Add => { if self.y < self.map.height - 1 { Some(self.y + 1) } else { None }},
                Operation::Sub => { if self.y > 0 { Some(self.y - 1) } else { None }}
            };
            
            if let (Some(x), Some(y)) = (x, y) {
                unsafe {
                    return Some(&mut self.map.data[y][x]);
                }
                
            }

            self.index += 1;
        }

        None
    }
}
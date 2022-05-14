use std::collections::BinaryHeap;


pub struct Chiton {
    maze: Vec<Vec<i32>>,
    expanded: Vec<Vec<i32>>
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Chiton {
        let maze = data
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|c| {
                        (c - b'0') as i32
                    }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
        
        let expanded = {
            (0..(5*maze.len())).map(|x| {
                (0..(5*maze[0].len())).map(|y| {
                    let cost = maze[x % maze.len()][y % maze[0].len()]
                        + (x / maze.len()) as i32
                        + (y / maze[0].len()) as i32;
                    if cost < 10 { cost } else { cost - 9}                
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        };
            
        Chiton { maze, expanded }
    }

    fn part1(&mut self) -> usize {
        shortest_path(&self.maze) as usize
    }

    fn part2(&mut self) -> usize {
        shortest_path(&self.expanded) as usize
    }
}

enum Arith {
    Sub,
    Add,
    None
}
const OFFSETS: [(Arith, Arith); 4] = [
    (Arith::Sub, Arith::None),
    (Arith::Add, Arith::None),
    (Arith::None, Arith::Sub),
    (Arith::None, Arith::Add)
];

fn shortest_path(maze: &[Vec<i32>]) -> i32 {
    let goal = (maze.len() - 1, maze[0].len() - 1);
    let mut dist = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
    let mut q = BinaryHeap::new();

    q.push((0, 0, 0));
    while let Some((cost, x, y)) = q.pop() {
        if (x, y) == goal { return -cost; }
        if -cost > dist[x][y] { continue; }

        for (x_a, y_a) in OFFSETS {
            let x1: Option<usize> = match x_a {
                Arith::None => Some(x),
                Arith::Add => { 
                    let rx = x + 1; 
                    if rx < maze.len() { Some(rx) } else { None }
                } ,
                Arith::Sub => if x > 0 { Some(x - 1)} else { None }
            };
            let y1: Option<usize> = match y_a {
                Arith::None => Some(y),
                Arith::Add => {
                    let ry = y + 1;
                    if ry < maze[0].len() { Some(ry) } else { None }
                },
                Arith::Sub => if y > 0 { Some(y - 1) } else { None }
            }; 

            
            if let (Some(x1), Some(y1)) = (x1, y1) {
                let c = maze[x][y];
                let next_cost = -cost + c;
                if next_cost < dist[x1][y1] {
                    q.push((-next_cost, x1, y1));
                    dist[x1][y1] = next_cost;
                }
            } else {
                continue;
            }
        }
    }

    unreachable!()    
}
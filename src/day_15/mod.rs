use std::collections::BinaryHeap;

pub struct Chiton {
    maze: Vec<Vec<i32>>,
    expanded: Vec<Vec<i32>>
}

impl crate::Advent for Chiton {
    fn new(data: &str) -> Chiton {
        let maze = data.lines()
            .map(|l| l.bytes().map(|c| (c - b'0') as i32).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let expanded = (0..(5*maze.len()))
            .map(|x| (0..(5*maze[0].len()))
            .map(|y| {
                let cost = maze[x % maze.len()][y % maze[0].len()]
                + (x / maze.len()) as i32
                + (y / maze[0].len()) as i32;
                if cost < 10 {cost} else {cost - 9}
            })
            .collect::<Vec<_>>()

            
            )
            .collect::<Vec<_>>();
        Chiton { maze, expanded }
    }

    fn part1(&mut self) -> usize {
        shortest_path(&self.maze) as usize
    }

    fn part2(&mut self) -> usize {
        shortest_path(&self.expanded) as usize
    }
}

fn shortest_path(maze: &[Vec<i32>]) -> i32 {
    let goal = (maze.len() - 1, maze[0].len() - 1);
    let mut dist = vec![vec![i32::MAX; maze[0].len()]; maze.len()];
    let mut q = BinaryHeap::new();

    q.push((0, 0, 0));
    while let Some((cost, x, y)) = q.pop() {
        if (x, y) == goal { return -cost; }
        if -cost > dist[x][y] { continue; }
        for (x1,y1) in [(x-1,y), (x+1,y), (x,y-1), (x,y+1)] {
            let next_cost = match maze.get(x1).and_then(|row| row.get(y1)) {
                Some(c) => -cost + c,
                None => continue,
            };
            if next_cost < dist[x1][y1] {
                q.push((-next_cost,x1,y1));
                dist[x1][y1] = next_cost;
            }
        }
    }

    unreachable!()    
}

// fn display(maze: &[Vec<i32>]) -> () {
//     for line in maze {    
//         for val in line {
//             print!("{:>2}", val);            
//         }
//         print!("\n");
//     }
// }

// fn display_cost(dist: &Vec<Vec<i32>>) -> () {
//     for line in dist {    
//         for cost in line {
//             if *cost == i32::MAX {
//                 print!("####");
//             } else {
//                 print!("{:>4}", cost);
//             }
            
//         }
//         print!("\n");
//     }
//     print!("\n");
// }
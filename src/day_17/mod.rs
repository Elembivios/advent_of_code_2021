use super::euclidean::{Coordinate, Axis};
use std::cmp;

type coord = Coordinate<i32>;

struct Area {
    start: coord,
    end: coord
}

impl Area {
    fn is_inside(&self, c: coord) -> bool {
        self.start.x < c.x && c.x < self.end.x && self.start.y < c.y && c.y < self.end.y
    }
}

struct Projectile {
    location: coord,
    vector: coord 
}

impl Projectile {
    fn new(vector: coord) -> Projectile {
        Projectile { 
            location: Coordinate { x:0, y:0},
            vector
        }
    }

}

pub struct TrickShot {
    target: Area
}

fn min_drag(min_dist: i32) -> i32 {
    let mut i = 0;
    let mut sum = 0;
    while sum <= min_dist {
        i+=1;
        sum += i;        
    }
    i
}

impl crate::Advent for TrickShot {
    fn new(data: &str) -> TrickShot {
        let (xs, ys) = data
            .strip_prefix("target area: ").unwrap()
            .split_once(", ").unwrap();
    
        let (xs1, xs2) = xs.strip_prefix("x=").unwrap().split_once("..").unwrap();
        let x1: i32 = xs1.parse().unwrap();
        let x2: i32 = xs2.parse().unwrap();

        let x1 = cmp::min(x1, x2);
        let x2 = cmp::max(x1, x2);

        let (ys1, ys2) = ys.strip_prefix("y=").unwrap().split_once("..").unwrap();
        let y1: i32 = ys1.parse().unwrap();
        let y2: i32 = ys2.parse().unwrap();

        let y1 = cmp::min(y1, y2);
        let y2 = cmp::max(y1, y2);

        let area = Area {
            start: Coordinate { x: x1, y: y1 },
            end: Coordinate { x: x2, y: y2 }
        };

        TrickShot { target: area }
    }



    fn part1(&mut self) -> usize {
        let min_x = min_drag(self.target.start.x);



        min_x as usize
    }

    fn part2(&mut self) -> usize {
        2
    }
}
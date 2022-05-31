use super::euclidean::Coordinate;
use std::cmp;

type Coord = Coordinate<i32>;

#[derive(Debug)]
struct Area {
    start: Coord,
    end: Coord
}

#[derive(Debug)]
struct Projectile {
    location: Coord,
    vector: Coord,
}

impl Projectile {    
    fn new(vector: Coord) -> Projectile {
        Projectile { 
            location: Coordinate { x:0, y:0},
            vector
        }
    }

    fn step(&mut self) -> () {
        self.location.y += self.vector.y;
        self.vector.y -= 1;
        
        let ord = self.vector.x.cmp(&0);
        match ord {
            cmp::Ordering::Greater => { self.location.x += self.vector.x; self.vector.x -= 1; },
            cmp::Ordering::Less => { self.location.x += self.vector.x; self.vector.x += 1; },
            _ => ()
        }        
    }

    fn overshot(&self, target: &Area) -> bool {
        if self.location.x > target.end.x {
            return true;
        }
        if self.vector.y < 0 && self.location.y < target.end.y {
            return true;
        }
        false
    }

    fn is_inside(&self, target: &Area) -> bool {
        (target.start.x..=target.end.x).contains(&self.location.x) && (target.end.y ..= target.start.y).contains(&self.location.y)
    }
}

pub struct TrickShot {
    target: Area
}

impl TrickShot {
    fn min_max(lhs: i32, rhs: i32) -> (i32, i32) {
        let ord = lhs.abs().cmp(&rhs.abs());
        match ord {
            cmp::Ordering::Greater => (rhs, lhs),
            _ => (lhs, rhs)        
        }
    }

    fn min_x(&self) -> i32 {
        let mut i = 0;
        let mut sum = 0;
        while sum <= self.target.start.x {
            i += 1;
            sum += i;
        }
        i
    }

    fn max_x(&self) -> i32 {
        self.target.end.x
    }

    fn max_y(&self) -> i32 {
        self.target.end.y.abs()
    }

    fn min_y(&self) -> i32 {
        self.target.end.y
    }
    
    fn max_height(&self) -> i32 {
        (1..self.max_y()).into_iter().sum()
    }
}

impl crate::Advent for TrickShot {
    fn new(data: &str) -> TrickShot {
        let (xs, ys) = data
            .strip_prefix("target area: ").unwrap()
            .split_once(", ").unwrap();
    
        let (xs1, xs2) = xs.strip_prefix("x=").unwrap().split_once("..").unwrap();
        let x1: i32 = xs1.parse().unwrap();
        let x2: i32 = xs2.parse().unwrap();

        let (x1, x2) = TrickShot::min_max(x1, x2);

        let x1 = cmp::min(x1, x2);
        let x2 = cmp::max(x1, x2);

        let (ys1, ys2) = ys.strip_prefix("y=").unwrap().split_once("..").unwrap();
        let y1: i32 = ys1.parse().unwrap();
        let y2: i32 = ys2.parse().unwrap();
        
        let (y1, y2) = TrickShot::min_max(y1, y2);

        let area = Area {
            start: Coordinate { x: x1, y: y1 },
            end: Coordinate { x: x2, y: y2 }
        };
        // println!("Area: {:?}", area);
        TrickShot { target: area }
    }



    fn part1(&mut self) -> usize {
        self.max_height() as usize
    }

    fn part2(&mut self) -> usize {
        let min_x = self.min_x();
        let max_x = self.max_x();
        let (min_x, max_x) = TrickShot::min_max(min_x, max_x);
        let max_y = self.max_y();
        let min_y = self.min_y();
        let (min_y, max_y) = TrickShot::min_max(min_y, max_y);
        let mut counter: usize = 0;
        for vec_x in min_x..=max_x {
            for vec_y in min_y..=max_y {    
                let vector = Coord{ x: vec_x, y: vec_y };
                let mut projectile = Projectile::new( vector );
                while !projectile.overshot(&self.target) {
                    projectile.step();
                    if projectile.is_inside(&self.target) {
                        counter += 1;
                        break;
                    }
                }
            }
        }
        counter
    }
}
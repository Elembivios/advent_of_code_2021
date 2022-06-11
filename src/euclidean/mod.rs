use std::cmp::{Ordering, PartialEq};
use std::ops::Index;
use std::fmt;

pub enum Axis {
    X,
    Y
}

#[allow(dead_code)]
#[derive(Eq, Hash, Debug)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T
}

impl<T: std::fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: PartialEq> PartialEq for Coordinate<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Ord> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);
        match y_cmp {
            Ordering::Equal => {
                self.x.cmp(&other.x)
            }
            _ => {
                y_cmp
            }
        }
    }    
}

impl<T: PartialOrd + Ord> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {        
        Some(self.cmp(other))
    }
}

impl Index<Axis> for Coordinate<u32> {
    type Output = u32;

    fn index(&self, axis: Axis) -> &Self::Output {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y
        }
    }
}

#[derive(Eq, Hash, Debug)]
pub struct Point<T, U> {
    pub val: T,
    pub coordinate: Coordinate<U>
}

#[allow(dead_code)]
impl<T, U> Point<T, U> {
    fn new(val: T, coordinate: Coordinate<U>) -> Point<T, U> {
        Point {val, coordinate}
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> fmt::Display for Point<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.val, self.coordinate)
    }
}

impl<T: PartialEq, U: PartialEq> PartialEq for Point<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.coordinate == other.coordinate
    }
}

impl<T: Ord, U: Ord> Ord for Point<T, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.val.cmp(&other.val)
        let ord = self.coordinate.cmp(&other.coordinate);
        if ord != Ordering::Equal {
            return ord;
        }
        self.val.cmp(&other.val)
    }
}

impl<T: PartialOrd + Ord, U: PartialOrd + Ord> PartialOrd for Point<T, U> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {        
        Some(self.cmp(other))
    }
}
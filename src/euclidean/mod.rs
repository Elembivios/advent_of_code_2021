use std::cmp::Ordering;
use std::ops::Index;
use std::fmt;

pub enum Axis {
    X,
    Y
}

#[derive(Eq)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T
}

impl<T: std::fmt::Display> fmt::Display for Coordinate<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::cmp::PartialEq> PartialEq for Coordinate<T> {
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

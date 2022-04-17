use std::cmp::Ordering;

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(s: &str) -> Self {
        let mut iter = s.split(',').flat_map(|c| c.parse());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Coordinate { x, y }
    }

    fn is_straight_with(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
}

pub struct Line {
    start: Coordinate,
    end: Coordinate
}

impl Line {
    fn is_diagonal(&self) -> bool {
        !self.start.is_straight_with(&self.end)
    }
}

pub struct HydrothermalVenture {
    lines: Vec<Line>,
}

impl crate::Advent for HydrothermalVenture {
    fn new(data: &str) -> HydrothermalVenture {
        let lines = data
            .lines()
            .map(|l| {
                let mut iter = l.split(" -> ").map(Coordinate::new);
                let start = iter.next().unwrap();
                let end = iter.next().unwrap();
                Line { start, end }
            })
            .collect();
        
        HydrothermalVenture { lines }
    }

    fn part1(&self) -> usize {
        count_overlap(self.lines.iter().filter(|l| !l.is_diagonal()))
    }

    fn part2(&self) -> usize {
        count_overlap(self.lines.iter())
    }
}

fn count_overlap<'a>(iter: impl Iterator<Item = &'a Line>) -> usize {
    let width = 1000;
    let mut map = vec![0u8; width * width];

    let middle_points = |a: usize, b: usize| match a.cmp(&b) {
        Ordering::Less => a + 1,
        Ordering::Greater => a - 1,
        _ => a,
    };

    iter.for_each(|l|{
        let mut x = l.start.x;
        let mut y = l.start.y;

        map[x + y * width] += 1;

        while x != l.end.x || y != l.end.y {
            x = middle_points(x, l.end.x);
            y = middle_points(y, l.end.y);
            
            map[x + y * width] += 1;
        }
    });

    map.into_iter().filter(|&v| v > 1).count()
}
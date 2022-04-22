
pub struct SmokeBasin {
    map: Vec<Coordinate>,
    height: usize,
    width: usize
}

#[derive(Debug)]
pub struct Coordinate {
    val: u8,
    x: usize,
    y: usize
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl SmokeBasin {
    fn get_coordinate_ref(&self, x: usize, y: usize) -> &Coordinate {
        return &self.map[self.width * y + x];
    }

    fn neighbours(&self, c: &Coordinate) -> Vec<&Coordinate> {
        let mut neighbours: Vec<&Coordinate> = Vec::new();
        if c.x > 0 {
            neighbours.push(self.get_coordinate_ref(c.x - 1, c.y));
        }
        if c.x < self.width - 1 {
            neighbours.push(self.get_coordinate_ref(c.x + 1, c.y));
        }
        if c.y > 0 {
            neighbours.push(self.get_coordinate_ref(c.x, c.y - 1));
        }
        if c.y < self.height - 1 {
            neighbours.push(self.get_coordinate_ref(c.x, c.y + 1));
        }
        neighbours
    }    

    fn low_points(&self) -> Vec<&Coordinate> {
        self.map
            .iter()
            .filter(|&c| {
                // No neighbour is larger than current point
                self.neighbours(c).iter().filter(|n| n.val <= c.val).count() == 0
            }).collect()
    }

    fn basin(&self, c: &Coordinate) -> Vec<&Coordinate> {
        let mut basin: Vec<&Coordinate> = vec![self.get_coordinate_ref(c.x, c.y)];
        let mut outer_coordinates = basin.clone();
        while outer_coordinates.len() != 0 {
            let mut new_outer_coordinates: Vec<&Coordinate> = vec![];
            outer_coordinates.iter().for_each(|outer| {
                self.neighbours(outer).iter().filter(|n| {
                    n.val != 9 && !basin.contains(n)
                }).for_each(|n| {
                    if !new_outer_coordinates.contains(n) {
                        new_outer_coordinates.push(n);
                    }                    
                });
            });            
            outer_coordinates = new_outer_coordinates;
            outer_coordinates.iter().for_each(|c| { basin.push(c)});
        }
        basin
    }    
}

impl crate::Advent for SmokeBasin {
    fn new(data: &str) -> SmokeBasin {      
        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();

        let map: Vec<Coordinate> = data
            .lines().enumerate()
            .flat_map(|l| {
                let (y, line) = l;
                line
                    .chars().enumerate()
                    .map(|c| {
                        let (x, chr) = c;                        
                        Coordinate { x, y, val: chr.to_digit(10).unwrap() as u8 }
                    }).collect::<Vec<Coordinate>>()                    
            })
            .collect();  
        SmokeBasin { map, height, width }
    }
 
    fn part1(&mut self) -> usize {            
        let low_points = self.low_points();
        let sum: usize = low_points.iter().copied().map(|c| c.val as usize).sum();
        sum + low_points.len()
    }
    
    fn part2(&mut self) -> usize {
        let low_points = self.low_points();
        let mut basins: Vec<Vec<&Coordinate>> = low_points.iter().map(|c| self.basin(c)).collect();
        basins.sort_unstable_by_key(|b| b.len());
        
        let mut result: usize = 1;
        basins.iter().rev().take(3).for_each(|c| result *= c.len());

        result
    }
}
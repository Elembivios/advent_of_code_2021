
pub struct SmokeBasin {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize
}

impl SmokeBasin {
    fn neighbours(&self, x: usize, y: usize) -> Vec<&u8> {
        let mut neighbours: Vec<&u8> = Vec::new();
        if x > 0 {
            neighbours.push(&self.map[y][x-1]);
        }
        if x < self.width - 1 {
            neighbours.push(&self.map[y][x+1]);
        }
        if y > 0 {
            neighbours.push(&self.map[y-1][x]);
        }
        if y < self.height - 1 {
            neighbours.push(&self.map[y + 1][x]);
        }
        neighbours
    }    

    fn low_points(&self) -> Vec<&u8> {
        self.map.iter().enumerate().flat_map(|r| {
            let (y, line) = r;
            line.iter().enumerate().filter(move | p| {
                let (x, value) = p;
                let neighbours = self.neighbours(*x, y);
                // No neighbour is larger than current point
                neighbours.iter().filter(|n| n <= &value).count() == 0
            }).map(|(_, value)| {
                value
            })
        }).collect()
    }
    
    fn basins(&self) -> () {
        let low_points: Vec<(usize, usize, &u8)> = self.map.iter().enumerate().flat_map(|r| {
            let (y, line) = r;
            line.iter().enumerate().filter(move | p| {
                let (x, value) = p;
                let neighbours = self.neighbours(*x, y);
                // No neighbour is larger than current point
                neighbours.iter().filter(|n| n <= &value).count() == 0
            }).map(move |(x, value)| {
                (x, y, value)
            })
        }).collect();
        println!("Low points: {:?}", low_points);

        

    }

}

impl crate::Advent for SmokeBasin {
    fn new(data: &str) -> SmokeBasin {        
        let map: Vec<Vec<u8>> = data
            .lines()
            .map(|line| {
                line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()                    
            })
            .collect();  

        let height = map.len();
        let width = map[0].len();
        SmokeBasin { map, height, width }
    }
 
    fn part1(&self) -> usize {        
        let low_points = self.low_points();
        let sum: usize = low_points.iter().copied().map(|&v| v as usize).sum();
        sum + low_points.len()
    }
    
    fn part2(&self) -> usize {
        self.basins();
        3
    }
}
pub struct SonarSweep {
    data: Vec<u32>,
}

impl crate::Advent for SonarSweep {
    fn new(data: &str) -> Self {
        SonarSweep {
            data: data.lines().filter_map(|l| l.parse().ok()).collect()
        }
    }

    fn part1(&mut self) -> usize {
        self.data 
            .iter()
            .zip(self.data.iter().skip(1))
            .filter(|(a, b)| a < b)
            .count()
    }

    fn part2(&mut self) -> usize {
        self.data
            .windows(3)
            .zip(self.data.windows(3).skip(1))
            .filter(|(a, b)| a.iter().sum::<u32>() < b.iter().sum())
            .count()
    }
}
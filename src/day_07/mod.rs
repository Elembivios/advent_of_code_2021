trait Math {
    fn is_even(&self) -> bool;

    /*
    Calculates triangle sum
    '''
    let sum = usize::triangle_sum(5);    
    asserteq!(sum, 15)
    '''
    1 + 2 + 3 + 4 + 5 = 15
    */
    fn triangle_sum(s: usize) -> usize {        
        (s + 1) * s / 2
    }
    fn abs_difference(&self, other: &usize) -> usize;

}

impl Math for usize {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }

    fn abs_difference(&self, other: &usize) -> usize {
        if self < other {
            other - self
        } else {
            self - other
        }
    }
}

pub struct TheThreacheryOfWhales {
    crab_positions: Vec<usize>
}

impl crate::Advent for TheThreacheryOfWhales {
    fn new(data: &str) -> TheThreacheryOfWhales {
        let mut crab_positions: Vec<usize> = data
            .split(',')
            .flat_map(|c| c.parse())
            .collect();
        crab_positions.sort_unstable();
        TheThreacheryOfWhales { crab_positions }
    }

    fn part1(&mut self) -> usize {
        let median = self.median();
        self.crab_positions.iter().map(|&p| p.abs_difference(&median)).sum()
    }

    fn part2(&mut self) -> usize {
        (self.mean()..)
            .take(2)
            .map(|m| {
                self.crab_positions
                    .iter()
                    .map(|&p| usize::triangle_sum(p.abs_difference(&m)))
                    .sum()
            })
            .min()
            .unwrap_or_default()
    }    
}

impl TheThreacheryOfWhales {
    fn median(&self) -> usize {
        let len = self.crab_positions.len();
        let middle = len / 2;
        if len.is_even() {
            (self.crab_positions[middle - 1] + self.crab_positions[middle]) / 2
        } else {
            self.crab_positions[middle] / 2
        }
    }

    fn mean(&self) -> usize {
        self.crab_positions.iter().sum::<usize>() / self.crab_positions.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn triangle_sum_10() {
        let sum = usize::triangle_sum(10);        
        assert_eq!(sum, 55)
    }

    #[test]
    fn devide_int_with_loss() {
        let lhs = 5;
        let rgs = 3;

        assert_eq!(lhs / rgs, 1)
    }
}
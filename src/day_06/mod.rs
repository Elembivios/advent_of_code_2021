use std::cell::RefCell;

pub struct Lanternfish {
    fish_by_age: RefCell<[usize; 9]>,
}

impl Lanternfish {
    fn pass_cycles(&self, days: usize) -> usize {
        let mut fish_by_age = self.fish_by_age.borrow_mut();
        for _ in 0..days {
            let new_fish = fish_by_age[0];
            (1..=8).for_each(|i| fish_by_age[i - 1] = fish_by_age[i]);
            fish_by_age[6] += new_fish;
            fish_by_age[8] = new_fish;
        }
        
        fish_by_age.iter().sum()
    }
}

impl crate::Advent for Lanternfish {
    fn new(data: &str) -> Lanternfish {
        let fish: Vec<u8> = data.split(",").flat_map(|f| f.parse()).collect();
        let mut fish_by_age = [0; 9];
        for f in fish {
            fish_by_age[f as usize] += 1;
        }
        Lanternfish { fish_by_age: RefCell::new(fish_by_age) }        
    }

    fn part1(&mut self) -> usize {
        self.pass_cycles(80)
    }

    fn part2(&mut self) -> usize {
        // Only pass additional cycles since we already passed 80 cycles
        self.pass_cycles(256 - 80)
    }
}
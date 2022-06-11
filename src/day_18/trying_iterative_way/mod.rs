mod snailfish_number;

use std::collections::VecDeque;
use snailfish_number::{El, Node};

pub struct Snailfish {
    numbers: Vec<El>
}

impl crate::Advent for Snailfish {
    fn new(data: &str) -> Snailfish {
        let numbers: Vec<El> = data.lines().map(|l| {            
            El::from_str(l)
        }).collect();
        // println!("Numbers: {:?}", numbers);
        Snailfish { numbers }
    }

    fn part1(&mut self) -> usize {
        println!("Number: {}", self.numbers[5]);
        1
    }

    fn part2(&mut self) -> usize {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inorder_iter() {
        let el = El::from_str("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        let items: Vec<_> = el.inorder_iter().collect();
        assert_eq!(items, vec![
            (&El::Nr(9), 3),
            (&El::Nr(3), 4),
            (&El::Nr(8), 4),
            (&El::Nr(0), 4),
            (&El::Nr(9), 4),
            (&El::Nr(6), 3),
            (&El::Nr(3), 4),
            (&El::Nr(7), 4),
            (&El::Nr(4), 4),
            (&El::Nr(9), 4),
            (&El::Nr(3), 2)
        ])
    }

    #[test]
    fn part1() {
        let mut el = El::from_str("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        for x in el.inorder_iter_mut() {
            println!("x: {}", x);
        }

    }


}
use bitvec::prelude::*;
use std::fmt;

struct Img {
    map: Vec<BitVec<u8, Msb0>>,
    edge: bool
}

impl fmt::Display for Img {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Image: \n")?;
        for bits in &self.map {
            let bits_str: String = bits.iter().map(|c| {
                match *c {
                    true => '⬜',
                    false => '⬛',
                }
            }).collect();
            write!(f, "{}\n", bits_str)?;
        }
        write!(f, "\n")
    }
}

use std::collections::VecDeque;

impl Img {
    fn enhance(&self, enhancment_algorithm: &BitVec<u8, Msb0>) -> Img {
        let padding = 1; // One padding on each side, 
        // we start from -1, -1 relative to original map and
        // end on width+1, height+1

        let new_edge = match self.edge {
            true => enhancment_algorithm[511],
            false => enhancment_algorithm[0]
        };
        let width = self.map[0].len();
        let height = self.map.len();
        let mut new_map: Vec<BitVec<u8, Msb0>> = Vec::with_capacity(height + padding * 2);

        let empty_row = VecDeque::from(
            [self.edge, self.edge, self.edge]
        );
        let empty_view: VecDeque<VecDeque<bool>> = VecDeque::from(
            [empty_row.clone(), empty_row.clone(), empty_row.clone()]
        );

        let mut view = empty_view;

        for y in 0..height + padding * 2 {    
            let mut new_row: BitVec<u8, Msb0> = BitVec::with_capacity(width + padding * 2);
            for x in 0..width + padding * 2 {
                // Insert new values to the right column of the view
                for offset_y in 0..=2 {
                    let oy = y.checked_sub(offset_y);                    
                    if let Some(oy) = oy {
                        let vy = 2 - offset_y;
                        view[vy].pop_front();
                        if oy < height && x < width {
                            view[vy].push_back(self.map[oy][x]);
                        } else {
                            view[vy].push_back(self.edge);
                        }
                    }
                }
                let bits: BitVec<u8, Msb0> = BitVec::from_iter(
                    view.iter().flatten()
                );
                let enhance_index: usize = bits.load_be();
                let new_value = enhancment_algorithm[enhance_index];
                new_row.push(new_value);
            }            
            new_map.push(new_row);

            // Reset the view
            for y in 0..3 {
                view[y].pop_front();
                view[y].push_back(self.edge);
            }
        }

        Img { map: new_map, edge: new_edge}
    }

}
pub struct TrenchMap {
    img_enhancment_algorithem: BitVec<u8, Msb0>,
    input_img: Img
}

impl crate::Advent for TrenchMap {
    fn new(data: &str) -> TrenchMap {
        let mut lines = data.lines();
        let img_enhancment_algorithem = lines
            .next().unwrap()
            .chars()
            .map(|c| {
                match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid char {}", c)
                }
            }).collect();

        lines.next();

        let mut input_img: Vec<BitVec<u8, Msb0>> = vec![];
        data.lines().skip(2).for_each(|l| {
            let mut bits = bitvec![u8, Msb0;];
            for c in l.chars() {
                match c {
                    '#' => bits.push(true),
                    '.' => bits.push(false),
                    _ => panic!("Invalid bit: {}", c)
                }
            }
            input_img.push(bits);
        });
        let input_img = Img { map: input_img, edge: false };
        // println!("{}", input_img);
        TrenchMap { 
            img_enhancment_algorithem,
            input_img
        }
    }

    fn part1(&mut self) -> usize {
        let img = self.input_img.enhance(&self.img_enhancment_algorithem);        
        let img = img.enhance(&self.img_enhancment_algorithem);
        // println!("{}", img);
        img.map.iter().flatten().filter(|bit| {
            **bit
        }).count()
    }

    fn part2(&mut self) -> usize {
        let mut img = self.input_img.enhance(&self.img_enhancment_algorithem); 
        for _ in 1..50 {
            img = img.enhance(&self.img_enhancment_algorithem);
        }               
        // println!("{}", img);
        img.map.iter().flatten().filter(|bit| {
            **bit
        }).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nine_digit_bits_to_int() {
        let bits = bitvec![u8, Msb0; 0, 1, 0, 0, 1, 0, 0, 0, 0];
        let num: usize = bits.load_be();
        println!("{:?}", bits);
        println!("Num: {}", num);

        let mut bits = bitvec![u8, Msb0;];
        bits.push(false);
        bits.push(true);
        bits.push(false);
        bits.push(false);
        bits.push(true);
        bits.push(false);
        bits.push(false);
        bits.push(false);
        bits.push(false);

        let num: usize = bits.load_be();
        println!("{:?}", bits);
        println!("Num: {}", num);

    }

    #[test]
    fn long_bitvec_indexing() {
        let bits_str = "#.#..##...###..#.#....#######.###..#.#.##.####.###.####.#..###..##..#..##.######...####..#.#...##...##.#####.#....##.###.##..#####....####..###..#.#......##....#####..#.###...###..##.#..#.#....##...#.#.#..#.###..###..#..#...##..##.###...###.......##.##..##.#.##.....####..##..#..##..#.##.##...##.#.##..###.#..##.#.##..######....#.##.#.........#..#.#.#..###..#.#...#....#.#.#..###.........####.#.###.#.####...#..#..#...#####.##..........#.##.#.#.###....#.#.#.#.#.....##...#.#...###...##...##.#..######..###.###.#.";
        let bits: BitVec<u8, Msb0> = bits_str.chars()
            .map(|c| {
                match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Invalid char {}", c)
                }
            }).collect();


        println!("len: {}", bits.len());

        println!("511: {:?}", bits.get(511));


        let mut it = bits.iter();
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), false);

        let mut it = bits.iter().rev();
        // "....#.#.#.#.#.....##...#.#...###...##...##.#..######..###.###.#."
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), false);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        assert_eq!(it.next().unwrap(), true);
        }

    #[test]
    fn test_empty_bitve() {
        let mut bits = bitvec![u8, Msb0;];
        for _ in 0..9 {
            bits.push(false);
        }
        println!("Bits: {:?}", bits);
        let num: usize = bits.load_be();
        println!("Num: {}", num);

    }
}

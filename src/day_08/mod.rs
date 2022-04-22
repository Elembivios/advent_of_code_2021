
pub struct SevenSegmentSearch {
    sequence: Vec<Display>,
}

type Pattern = Vec<char>;

#[derive(Debug)]
pub struct Display {
    signal_patterns: Vec<Pattern>,
    output_values: Vec<Pattern>
}

impl Display {
    fn get_unique(&self, len: usize) -> &Pattern {
        self.signal_patterns
            .iter()
            .filter(|s| {
                s.len() == len
            }).next().unwrap()
    }

    fn get_several(&self, len: usize) -> Vec<&Pattern> {
        self.signal_patterns
            .iter()
            .filter(|s| s.len() == len)
            .collect()
    }

    fn decode_signal_patterns(&self) -> (char, char, char) {
        // Decodes signal patterns to a point that we can decode the output values
        // Returns necesarry segment values for decoding output values

        let one_segment = self.get_unique(2);

        // Determine which segment corresponds to the 'c' and 'f' segments
        // of the 6-segment digits, two use 'c', one doesn't
        // Compare to the digit one, and both 'c' and 'f' segments can be determined        
        let six_segments = self.get_several(6);
        let (c_seg, f_seg) = {
            let test_char = one_segment[0];
            let count = six_segments.iter().filter(|s| s.contains(&test_char)).count();
            match count {
                2 => (test_char, one_segment[1]), // Tested char is 'f', the other is 'c'
                3 => (one_segment[1], test_char), // Oposite case
                _ => panic!("Comparing 1 digit to all with length 6 returned invalid result!")
            }
        };

        // Determin which segment corresponds to the 'e' segment
        // using the 'c' and 'f' segments, one can distinguish between 2, 3 and 5
        // Find the 3 and 2, then the 'e' segment is the one that is present in 2 but not in 3
        let five_segments = self.get_several(5);
        let three_segment = five_segments.iter().filter(|s| s.contains(&c_seg) && s.contains(&f_seg)).next().unwrap();
        let two_segment = five_segments.iter().filter(|s| s.contains(&c_seg) && !s.contains(&f_seg)).next().unwrap();
        let e_seg = two_segment.iter().filter(|c| !three_segment.contains(&c)).next().unwrap().clone();

        // The three obtained segments are enough to distinguish between the non-unique-length digits
        (c_seg, f_seg, e_seg)
    }

    fn decode_output(&self) -> u32 {
        let (c_seg, f_seg, e_seg) = self.decode_signal_patterns();

        let decode_digit = |digit: &Pattern| -> u32 {
            match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    if digit.contains(&c_seg) && digit.contains(&f_seg) {
                        3
                    } else if digit.contains(&c_seg) {
                        2
                    } else {
                        5
                    }
                },
                6 => {
                    if !digit.contains(&c_seg) {
                        6
                    } else if digit.contains(&e_seg) {
                        0
                    } else {
                        9
                    }
                }
                _ => panic!("Invalid length of sequence of charactes / borked input!")

            }
        };

        let mut digits: Vec<u32> = self.output_values.iter().map(decode_digit).collect();
        // Multiply each digit by it's decimal place  so we get the whole number
        digits[0] *= 1000;
        digits[1] *= 100;
        digits[2] *= 10;
        digits.iter().sum()
    }
}


impl crate::Advent for SevenSegmentSearch {
    fn new(data: &str) -> SevenSegmentSearch {
        let sequence = data
            .lines()
            .map(|s| {
                let mut iter = s.split(" | ");
                let signal_patterns: Vec<Pattern> = iter.next().unwrap().split(' ').map(|s| s.chars().collect()).collect();
                let output_values: Vec<Pattern> = iter.next().unwrap().split(' ').map(|s| s.chars().collect()).collect();
                let d = Display { signal_patterns, output_values };
                //println!("Display: {:?}", d);
                d

            })
            .collect();

        // let numbers_length_map = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
        SevenSegmentSearch { sequence, }
    }

    fn part1(&mut self) -> usize {
        self.sequence
            .iter()
            .map(|d| {
                d.output_values
                    .iter()
                    .filter(|v| {
                        match v.len() {
                            2 => true,
                            3 => true,
                            4 => true,
                            7 => true,
                            _ => false
                        }
                    }).count()
            })
            .sum()
    }

    fn part2(&mut self) -> usize {    

        //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

        // a = [0,    2, 3,    5, 6, 7, 8, 9]
        // b = [0,          4, 5, 6,    8, 9]
        // c = [0, 1, 2, 3, 4,       7, 8, 9]
        // d = [      2, 3, 4, 5, 6,    8, 9]
        // e = [0,    2,          6,    8]
        // f = [0, 1,    3, 4, 5, 6, 7, 8, 9]
        // g = [0,    2, 3,    5, 6,    8, 9]

        //     [6, 2, 5, 5, 4, 5, 6, 3, 7, 6]
        

        let result: u32 = self.sequence
            .iter()
            .map(|d| {
                d.decode_output()
            })
            .sum();
        
        result as usize
    }
}
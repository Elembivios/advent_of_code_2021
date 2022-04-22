

pub struct BinaryDiagnostic {
    data: Vec<Bin>,
    mid_point: f32
}

type Bin = Vec<u8>;

fn vec_to_int(v: &Bin) -> usize {
    v.iter().fold(0, |acc, &b| acc * 2 + b as usize)
}

impl crate::Advent for BinaryDiagnostic {
    fn new(data: &str) -> BinaryDiagnostic {        
        BinaryDiagnostic {        
            data: data
                .lines()
                .map(|l| {
                    l
                        .chars()
                        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                        .collect::<Bin>()                
                })
                .collect(),
            mid_point: data.lines().count() as f32 / 2.0
            }
    }

    fn part1(&mut self) -> usize {
        let columns_sum = self.get_columns_sum();
        let mut gamma_rating: Bin = Vec::new();
        for column_sum in columns_sum.iter() {
            if *column_sum as f32 > self.mid_point {
                gamma_rating.push(1);
            } else {
                gamma_rating.push(0);
            }
        }

        let mut epsilon_rating: Bin = Vec::new();

        for bit in &gamma_rating {
            match bit {
                0 => epsilon_rating.push(1),
                1 => epsilon_rating.push(0),
                _ => panic!("Invalid gamma rating {}!", bit)
            }
        }

        let gamma: usize = vec_to_int(&gamma_rating);
        let epsilon: usize = vec_to_int(&epsilon_rating);

        gamma * epsilon
    }

    fn part2(&mut self) -> usize {
        let oxygen_rating = self.get_life_support_rating_part(true);
        let co2_scrubber_rating = self.get_life_support_rating_part(false);

        let oxygen = vec_to_int(&oxygen_rating);
        let co2_scrubber = vec_to_int(&co2_scrubber_rating);

        oxygen * co2_scrubber
    }
}

impl BinaryDiagnostic {
    
    fn get_common_bit(&self, data: &Vec<Bin>, index: usize, common: bool) -> u8 {
        let bit: f32 = data
            .iter()
            .filter(|x| x[index] == 1)
            .count() as f32;
        let mid_point: f32 = data.len() as f32 / 2.0;
        let compare_func = match common {
            true => |bit, mid_point| bit >= mid_point,
            false => |bit, mid_point| bit < mid_point
        };
        if compare_func(bit, mid_point) { 1 } else { 0 }                 
    }

    fn get_columns_sum(&self) -> Vec<u32> {
        let mut columns_sum: Vec<u32> = vec![0; self.data[0].len()];
        for row in &self.data {
            for (i, bit) in row.iter().enumerate() {
                columns_sum[i] += *bit as u32;
            }
        }

        columns_sum
    }

    fn get_life_support_rating_part(&self, common: bool) -> Bin {
        let mut rating: Bin = Vec::new();
        let mut new_data: Vec<Bin> = self.data.clone();
        
        for i in 0..self.data[0].len() {
            let common_bit = self.get_common_bit(&new_data, i, common);
            rating.push(common_bit);

            let temp = new_data
                .into_iter()
                .filter(|x| x[0..rating.len()] == rating)
                .collect();
            new_data = temp;
            if new_data.len() <= 1 {
                return new_data[0].clone();
            }
        }

        panic!("Reached the end!")
    }
}
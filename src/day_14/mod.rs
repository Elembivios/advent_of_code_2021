use std::collections::HashMap;
use std::fmt;

pub struct ExtendedPolymerization {
    insertion_rules: Vec<InsertionRule>,
    pair_count: HashMap<[char; 2], usize>
}

impl ExtendedPolymerization {
    fn run_rules(&mut self) {
        let mut new_pair_count= self.pair_count.clone();
        for ([lhs, rhs], count) in &self.pair_count {
            if let Some(insertion_rule) = self.insertion_rules.iter().find(|rule| rule.input == [*lhs, *rhs]) {
                let lhs_key = [*lhs, insertion_rule.output];
                let rhs_key = [insertion_rule.output, *rhs];
                
                new_pair_count.entry(lhs_key).and_modify(|v| *v += count).or_insert(*count);
                new_pair_count.entry(rhs_key).and_modify(|v| *v += count).or_insert(*count);
                new_pair_count.entry(insertion_rule.input).and_modify(|val| *val -= count);
            }
        }
        self.pair_count = new_pair_count;
    }

    fn occurances(&self) -> HashMap<char, f64> {
        let mut occurances: HashMap<char, f64> = HashMap::new();
        for (key, val) in &self.pair_count {            
            for side in key {
                let real_val =  *val as f64 / 2_f64;
                occurances.entry(*side).and_modify(|v| *v += real_val).or_insert(real_val);
            }
        }
        occurances.iter_mut().for_each(|(_, v)| *v = v.ceil());

        occurances
    }

    fn min_max(&self, occurances: HashMap<char, f64>) -> usize {        
        let values: Vec<usize> = occurances.iter().map(|(_, v)| *v as usize).collect();
        let min_val = values.iter().min().unwrap().clone();
        let max_val = values.iter().max().unwrap().clone();
        max_val - min_val
    }
}

impl fmt::Display for ExtendedPolymerization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current count:\n")?;
        for (k, v) in self.pair_count.iter().filter(|(_, v)| **v > 0) {
            write!(f, "{:?} -> {}\n", k, v)?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug)]
struct InsertionRule {
    input: [char; 2],
    output: char
}

fn construct_pairs_count(template: Vec<char>) -> HashMap<[char; 2], usize> {
    let pairs_iter = template.windows(2);
    let mut pair_count = HashMap::new();
    for pair_s in pairs_iter {
        let pair: [char; 2] = [pair_s[0], pair_s[1]];
        let count = pair_count.entry(pair).or_insert(0);
        *count += 1;            
    }
    pair_count
}

impl crate::Advent for ExtendedPolymerization {
    fn new(data: &str) -> ExtendedPolymerization {
        let polymer_template: Vec<char> =  data.lines().next().unwrap().chars().collect();
        let insertion_rules =  data.lines().skip(2).map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            InsertionRule {
                input: input.chars().collect::<Vec<char>>().try_into().unwrap(),
                output: output.chars().next().unwrap()
            }
        }).collect();             
        let pair_count = construct_pairs_count(polymer_template);

        ExtendedPolymerization { insertion_rules, pair_count }
    }

    fn part1(&mut self) -> usize {
        for _ in 0..10  {
            self.run_rules();
        }            
        let occurances = self.occurances();
        self.min_max(occurances)
        
    }
    
    fn part2(&mut self) -> usize {
        for _ in 0..30 {
            self.run_rules();
        }

        let occurances = self.occurances();
        self.min_max(occurances)
    }
}
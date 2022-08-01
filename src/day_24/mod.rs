// use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::{collections::HashMap};

#[derive(Debug, Clone)]
enum Operation {
    Inp,
    Add,
    Mul,
    Div,
    Rem,
    Eq
}

impl Operation {
    fn from_str(s: &str) -> Self {
        match s {
            "inp" => Self::Inp,
            "add" => Self::Add,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "mod" => Self::Rem,
            "eql" => Self::Eq,
            _ => panic!("Invalid string {} for operations.", s)
        }
    }
    fn calculate(&self, lhs: &mut i64, rhs: i64) {
        match self {
            Self::Inp => *lhs = rhs,
            Self::Add => *lhs += rhs,
            Self::Mul => *lhs *= rhs,
            Self::Div => *lhs /= rhs,
            Self::Rem => *lhs %= rhs,
            Self::Eq => {
                if *lhs == rhs {
                    *lhs = 1
                } else {
                    *lhs = 0
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Arg {
    Value(i64),
    Variable(char)
}

// fn number_to_vec(n: u64) -> Vec<u8> {
//     let mut digits= Vec::new();
//     let mut n = n;
//     while n > 9 {
//         digits.push((n % 10)as u8);
//         n = n / 10;
//     }
//     digits.push(n as u8);
//     digits.reverse();

//     digits
// }

pub struct ArithemticLogicUnit {
    operations: Vec<(Operation, Vec<Arg>)>,
}

impl crate::Advent for ArithemticLogicUnit {
    fn new(data: &str) -> ArithemticLogicUnit {
        let operations: Vec<(Operation, Vec<Arg>)> = data.lines().map(|l| {
            let mut parts = l.split(" ");
            let operation = Operation::from_str(parts.next().unwrap());
            let arguments: Vec<Arg> = parts.map(|a| {
                let res: Result<i64, _> = a.parse();
                if let Ok(val) = res {
                    Arg::Value(val)
                } else {
                    Arg::Variable(a.chars().next().unwrap())
                }
            }).collect();
            (operation, arguments)            
        }).collect();
        println!("{:?}", operations);


        ArithemticLogicUnit { operations }   
    }    

    

    fn part1(&mut self) -> usize {
        let chunks = self.operation_chunks();

        for chunk in chunks.iter() {
            println!("Chunk: \n\t{:?}\n", chunk);
        }
        let result = compute_model_number(0, 0, &chunks, &mut HashMap::new());
        println!("Result: {:?}", result);
        
        1
    }

    fn part2(&mut self) -> usize {
        2
    }
}

fn compute_model_number(number: u64, position: u8, operations: &Vec<Vec<(Operation, Vec<Arg>)>>, memo: &mut HashMap<u64, HashMap<char, i64>>) -> Option<HashMap<char, i64>> {    
    
    if position == 8 {
        println!("Number: {:?}, Position: {}", number, position);
        println!("Memo: {:?}", memo.len());
        println!("Keys: {:?}", memo.keys());
    }
    let mut variables: HashMap<char, i64> = if position == 0 {
        HashMap::new()
    } else {
        let previous_key = {
            if number > 10 {
                number / 10
            } else {
                number
            }
        };
        memo.get(&previous_key).unwrap().clone()
    };
    if position as usize >= operations.len() {
        let z = variables.get(&'z').unwrap();
        if *z == 0 {
            return Some(variables);
        } else {
            return None;
        }
    }
    for x in (1..=9).rev() {
        for (operation, arguments) in operations[position as usize].iter() {
            match &arguments[0] {
                Arg::Variable(lhs_c) => {
                    match operation {
                        Operation::Inp => {
                            let rhs = number % 10;
                            variables.insert(*lhs_c, rhs as i64);
                        }, 
                        _ => {
                            match &arguments[1] {
                                Arg::Variable(rhs_c) => {
                                    let mut lhs = *variables.get(&lhs_c).unwrap_or(&0);
                                    let rhs = variables.get(&rhs_c).unwrap_or(&0);
                                    operation.calculate(&mut lhs, *rhs);
                                    variables.insert(*lhs_c, lhs);
                                },
                                Arg::Value(rhs_v) => {
                                    let lhs = variables.entry(*lhs_c).or_insert(0);
                                    operation.calculate(lhs, *rhs_v);
                                }
                            }
                        }
                    }
                },
                _ => unreachable!()
            }
        }
    
        let key = number * 10 + x;
        memo.insert(key, variables.clone());
        let result = compute_model_number(key, position+1, operations, memo);
        if result.is_some() {
            return result;
        }
    }
    None
}

impl ArithemticLogicUnit {
    fn operation_chunks(&self) -> Vec<Vec<(Operation, Vec<Arg>)>> {
        let mut operation_iter = self.operations.clone().into_iter().peekable();

        let mut chunks: Vec<Vec<(Operation, Vec<Arg>)>> = vec![];
        let mut chunk: Vec<_> = vec![];
        let mut seen_inp = false;

        while operation_iter.peek().is_some() {            
            let next_element = operation_iter.next_if(|(op, _args)| {
                match op {
                    Operation::Inp => {
                        if seen_inp {
                            return false;
                        } else {
                            seen_inp = true;
                            return true;
                        }
                    }
                    _ => true
                }
            });

            match next_element {
                Some(el) => chunk.push(el),
                None => {
                    chunks.push(chunk.clone());
                    seen_inp = false;
                    chunk.clear();
                }
            }
        }
        
        chunks
    }    
}
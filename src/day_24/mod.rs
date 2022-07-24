// use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

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
    fn calculate(&self, lhs: &mut i32, rhs: i32) {
        match self {
            Inp => *lhs = rhs,
            Add => *lhs += rhs,
            Mul => *lhs *= rhs,
            Div => *lhs /= rhs,
            Mod => *lhs %= rhs,
            Eq => {
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
    Value(i32),
    Variable(char)
}

fn number_to_vec(n: u64) -> Vec<u8> {
    let mut digits= Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push((n % 10)as u8);
        n = n / 10;
    }
    digits.push(n as u8);
    digits.reverse();

    digits
}

pub struct ArithemticLogicUnit {
    operations: Vec<(Operation, Vec<Arg>)>,
    model_number: u64
}

impl crate::Advent for ArithemticLogicUnit {
    fn new(data: &str) -> ArithemticLogicUnit {
        let operations: Vec<(Operation, Vec<Arg>)> = data.lines().map(|l| {
            let mut parts = l.split(" ");
            let operation = Operation::from_str(parts.next().unwrap());
            let arguments: Vec<Arg> = parts.map(|a| {
                let res: Result<i32, _> = a.parse();
                if let Ok(val) = res {
                    Arg::Value(val)
                } else {
                    Arg::Variable(a.chars().next().unwrap())
                }
            }).collect();
            (operation, arguments)            
        }).collect();
        println!("{:?}", operations);

        let model_number = 13579246899999;

        ArithemticLogicUnit {
            operations,
            model_number
        }   
    }

    fn part1(&mut self) -> usize {
        let mut operations = self.operations.clone();
        let mut model_number = number_to_vec(self.model_number).into_iter();
        let inputs = operations.iter_mut().filter(|(operation, _)| {
            match operation {
                Operation::Inp => true,
                _ => false
            }
        }).map(|(_operation, arguments)| {
            arguments.push(Arg::Value(model_number.next().unwrap() as i32))
        });

        
        1
    }

    fn part2(&mut self) -> usize {
        2
    }
}
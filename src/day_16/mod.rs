
use hex;
use bitvec::prelude::*;

pub struct PacketDecoder {
    // bits: BitVec<u8, Msb0>
    version_sum: usize,
    equation: Vec<Packet>
}

#[derive(Debug, Clone)]
enum Remainder {
    Length(usize), // Bits value represents the length of subpackets
    Number(usize) // Bits value represents number of subpackets
}

impl Remainder {
    fn val(&self) -> usize {
        match self {
            Self::Length(val) => *val,
            Self::Number(val) => *val
        }
    }
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal
}

#[derive(Debug)]
struct Operator {
    operation: Operation,
    remainder: Remainder,
    length: usize
}

#[derive(Debug)]
struct Number {
    value: usize,
    length: usize
}

#[derive(Debug)]
enum Packet {
    Op(Operator),
    Num(Number)
}

fn parse_data(data: &BitSlice<u8, Msb0>) -> (usize, Vec<Packet>) {    
    let mut i: usize = 0; // Pointer to the current index
    let mut version_sum: usize = 0;
    let mut equation: Vec<Packet> = vec![];

    while data[i..].len() > 10 {            
        let version = &data[i .. i + 3];
        version_sum += version.load_be::<usize>();
        i += 3;
        let type_id = &data[i .. i + 3];
        i += 3;
        match type_id.load_be::<u8>() {
            4 => {
                let start = i.clone();
                let mut value_bits =  bitvec![u8, Msb0;];                
                for chunk in data[i ..].chunks(5) {
                    i+= 5;
                    let continues = chunk[0];
                    let value = &chunk[1 .. 5];
                    value_bits.extend_from_bitslice(value);
                    if continues == false {
                        break;
                    }                        
                }
                let number = Number { value: value_bits.load_be(), length: i - start + 6};    
                let packet = Packet::Num(number);
                equation.push(packet);
            },
            type_id => {
                let operation = match type_id {
                    0 => Operation::Sum,
                    1 => Operation::Product,
                    2 => Operation::Minimum,
                    3 => Operation::Maximum,
                    5 => Operation::Greater,
                    6 => Operation::Less,
                    7 => Operation::Equal,
                    _ => panic!("Invalid operator type")
                };
                let length_type_id = &data[i];
                i += 1;
                match length_type_id {
                    true => {
                        let number_of_packets = data[i .. i + 11].load_be::<usize>();
                        i += 11;
                        let remainder = Remainder::Number(number_of_packets);
                        let operator = Operator { 
                            operation, 
                            remainder,
                            length: 11 + 7
                        };                            
                        equation.push(Packet::Op(operator));
                    },
                    false => {
                        let total_length = data[i .. i + 15].load_be::<usize>();
                        i += 15;
                        let remainder = Remainder::Length(total_length);
                        let operator = Operator { 
                            operation,
                            remainder,
                            length: 15 + 7
                        };
                        equation.push(Packet::Op(operator));
                    }
                }
            }
        }            
    }

    (version_sum, equation)
}

impl crate::Advent for PacketDecoder {
    fn new(data: &str) -> PacketDecoder {
        let line = data
            .lines()
            .next()
            .unwrap();
        let hex = hex::decode(line);              
        // println!("Hex data: {:?}", hex);
        let bit_vec = if let Ok(hex) = hex {
            BitVec::<_, Msb0>::try_from_vec(hex).unwrap()
        } else {
            BitVec::new()
        };

        // println!("Bit vec: {:?}", bit_vec);

        let (version_sum, equation) = parse_data(&bit_vec);
        
        // println!("Equation: {:?}", equation);

        PacketDecoder { version_sum, equation }
    }

    fn part1(&mut self) -> usize {        
        self.version_sum
    }

    fn part2(&mut self) -> usize {                        
        while self.equation.len() > 1 {   
            let mut iterator = self.equation.iter().enumerate();        

            let mut all_numbers_index: Option<(usize, usize)> = None;
            'outer: while let Some((i, packet)) = iterator.next() {            
                match packet {
                    Packet::Op(operator) => {
                        let mut length_sum: usize = 0;
                        let mut num_subpackets: usize = 0;
                        let mut child_index: usize = i + 1;
                        while length_sum < operator.remainder.val() {   
                            let next = self.equation.get(child_index);
                            child_index += 1;                            
                            if let Some(next) = next {
                                match next {
                                    Packet::Num(num) => {
                                        match operator.remainder {
                                            Remainder::Number(_) => length_sum += 1,
                                            Remainder::Length(_) => length_sum += num.length                                    
                                        }                                    
                                    },
                                    Packet::Op(_) => { continue 'outer; }
                                }
                                num_subpackets += 1;                    
                            } else {
                                continue 'outer;
                            }
                        }         
                        all_numbers_index = Some((i, num_subpackets));            
                        
                    },
                    _ => { continue 'outer; }

                }
                if let Some(_) = all_numbers_index {
                    break 'outer
                }
            } 
            if let Some((all_numbers_index, num_subpackets)) = all_numbers_index {
                let mut value: usize = 0;
                let mut length_sum: usize = 0;
                let packet = &self.equation[all_numbers_index];
                match packet {
                    Packet::Op(operator) => {                        
                        let numbers: Vec<&Number> = self.equation[all_numbers_index + 1 .. all_numbers_index + 1 + num_subpackets].iter().filter_map(|p| {
                            match p {
                                Packet::Num(num) => Some(num),
                                Packet::Op(_) => None
                            }
                        }).collect();
                        let values: Vec<usize> = numbers.iter().map(|n| n.value).collect();
                        length_sum += numbers.iter().map(|n| n.length).sum::<usize>();
                        length_sum += operator.length;
                        value = match operator.operation {
                            Operation::Sum => values.iter().sum(),
                            Operation::Product => values.iter().product(),
                            Operation::Minimum => *values.iter().min().unwrap(),
                            Operation::Maximum => *values.iter().max().unwrap(),
                            Operation::Greater => if values[0] > values[1] { 1 } else { 0 },
                            Operation::Less => if values[0] < values[1] { 1 } else { 0 },
                            Operation::Equal => if values[0] == values[1] { 1 } else { 0 },                        
                        }                    
                    },
                    _ => panic!("This should be an operator")
                }        

                let number = Number { value, length: length_sum };

                let packet = self.equation.get_mut(all_numbers_index).unwrap();
                *packet = Packet::Num(number);

                self.equation.drain(all_numbers_index + 1 .. all_numbers_index + 1 + num_subpackets);
            }
            

        }
        match &self.equation[0] {
            Packet::Num(num) => num.value,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_bit() {
        let byte: u8 = 32;
        assert_eq!(byte.count_ones(), 1);
    }
}
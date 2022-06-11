use std::ascii::AsciiExt;
use std::ops::ControlFlow;
use std::fmt;

enum SnailSymbol {
    LBracket,
    RBracket,
    Comma,
    Num(u8)
}

impl SnailSymbol {
    fn from_char(c: char) -> SnailSymbol {
        match c {
            '[' => SnailSymbol::LBracket,
            ']' => SnailSymbol::RBracket,
            ',' => SnailSymbol::Comma,
            x => SnailSymbol::Num(x.to_digit(10).unwrap() as u8)
        }
    }

    fn to_char(&self) -> char {
        match self {
            SnailSymbol::LBracket => { '[' }
            SnailSymbol::RBracket => { ']' }
            SnailSymbol::Comma => { ',' }
            SnailSymbol::Num(x) => { std::char::from_digit(*x as u32, 10).unwrap() }
        }
    }
}

impl fmt::Display for SnailSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

type SnailfishNumber = Vec<SnailSymbol>;

fn parse_snail(s: &str) -> SnailfishNumber {
    s.chars().map(|c| SnailSymbol::from_char(c)).collect()
}

fn explode(number: &mut SnailfishNumber) -> ControlFlow<()> {
    let mut depth = 0;
    let explode_pos = number.iter().position(|sym| {
        match sym {
            SnailSymbol::LBracket => depth += 1,
            SnailSymbol::RBracket => depth -= 1,
            _ => ()
        }
        depth == 5
    });

    if let Some(explode_pos) = explode_pos {
        let lhs = match number[explode_pos + 1] {
            SnailSymbol::Num(lhs) => lhs,
            _ => unreachable!("Invalid format")
        };
        let rhs = match number[explode_pos + 3] {
            SnailSymbol::Num(rhs) => rhs,
            _ => unreachable!("Invalid format")        
        };
        
        number[..explode_pos].iter_mut().rev().find_map(|s| {
            if let SnailSymbol::Num(s) = s {
                Some(s)
            } else {
                None
            }
        }).map(|s| *s += lhs);

        number[explode_pos + 4 ..].iter_mut().rev().find_map(|s| {
            if let SnailSymbol::Num(s) = s {
                Some(s)
            } else {
                None
            }
        }).map(|s| *s += rhs);

        number[explode_pos] = SnailSymbol::Num(0);
        number.drain(explode_pos + 1 .. explode_pos + 5);
        ControlFlow::Break(())
    } else {
        ControlFlow::Continue(())
    }
}

fn split(number: &mut SnailfishNumber) -> ControlFlow<()> {
    let split_pos = number.iter().position(|s| {
        if let SnailSymbol::Num(x) = s {
            *x >= 10
        } else {
            false
        }
    });
    if split_pos.is_none() {
        return ControlFlow::Continue(())
    }
    let split_pos = split_pos.unwrap();

    let num = match number[split_pos] {
        SnailSymbol::Num(x) => x,
        _ => unreachable!()
    };

    let lhs = num / 2;
    let rhs = num - lhs;

    // number[split_pos] = SnailSymbol::
    number.splice(split_pos .. split_pos, [
        SnailSymbol::LBracket,
        SnailSymbol::Num(lhs),
        SnailSymbol::Comma,
        SnailSymbol::Num(rhs),
        SnailSymbol::RBracket
    ]);

    ControlFlow::Break(())
}


fn reduce(number: &mut SnailfishNumber) {
    let mut flow: ControlFlow<()> = ControlFlow::Continue(());
    while flow.is_continue() {
        explode(number);

        flow = split(number)
    }
}

pub struct Snailfish {
    numbers: Vec<SnailfishNumber>
}

impl crate::Advent for Snailfish {
    fn new(data: &str) -> Snailfish {
        let numbers: Vec<SnailfishNumber> = data.lines().map(|l| {
            parse_snail(l)
        }).collect();

        Snailfish { numbers }
    }

    fn part1(&mut self) -> usize {

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
    fn test_creation() {
        let number = parse_snail("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        println!("{}", number.into_iter().map(|s| s.to_char()).collect::<String>());
    }
}
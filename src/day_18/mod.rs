
use std::ops::ControlFlow;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[cfg(test)]
fn parse_snail_double_digit(s: &str) -> SnailfishNumber {
    let mut snail_number: SnailfishNumber = vec![];
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;    
    while i < chars.len() {
        let current = SnailSymbol::from_char(chars[i]);
        let next = if i < chars.len() - 1 {
            Some(SnailSymbol::from_char(chars[i + 1]))
        } else {
            None
        };

        match (current, next) {
            (current, None) => {
                snail_number.push(current);
                i += 1;
            },
            (SnailSymbol::Num(lhs), Some(SnailSymbol::Num(rhs))) => {
                let symbol = SnailSymbol::Num( (lhs * 10) + rhs );
                snail_number.push(symbol);
                i += 2;
            },
            (current, Some(SnailSymbol::Num(_))) => {
                snail_number.push(current);
                i += 1;
            },
            (current, Some(next)) => {
                snail_number.push(current);
                snail_number.push(next);
                i += 2;
            }
        }        
    }

    snail_number
}

fn add_snailnum(lhs: &[SnailSymbol], rhs: &[SnailSymbol], output: &mut SnailfishNumber) {
    output.push(SnailSymbol::LBracket);
    output.extend(lhs.iter());
    output.push(SnailSymbol::Comma);
    output.extend(rhs.iter());
    output.push(SnailSymbol::RBracket);
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

        number[explode_pos + 4 ..].iter_mut().find_map(|s| {
            if let SnailSymbol::Num(s) = s {
                Some(s)
            } else {
                None
            }
        }).map(|s| *s += rhs);

        number[explode_pos] = SnailSymbol::Num(0);
        number.drain(explode_pos + 1 .. explode_pos + 5);
        ControlFlow::Continue(())
    } else {
        ControlFlow::Break(())
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
        return ControlFlow::Break(())
    }
    let split_pos = split_pos.unwrap();

    let num = match number[split_pos] {
        SnailSymbol::Num(x) => x,
        _ => unreachable!()
    };

    let lhs = num / 2;
    let rhs = num - lhs;

    number[split_pos] = SnailSymbol::RBracket;
    number.splice(split_pos .. split_pos, [
        SnailSymbol::LBracket,
        SnailSymbol::Num(lhs),
        SnailSymbol::Comma,
        SnailSymbol::Num(rhs),
    ]);

    ControlFlow::Continue(())
}

fn reduce(number: &mut SnailfishNumber) {
    let mut flow: ControlFlow<()> = ControlFlow::Continue(());
    while flow.is_continue() {
        let temp_flow = explode(number);
        if temp_flow.is_continue() {
            continue;
        }
        flow = split(number)
    }
}

fn magnitude(number: &SnailfishNumber) -> usize {
    let mut multiplier = 1;
    let mut output = 0;
    for symbol in number {
        match symbol {
            SnailSymbol::LBracket => multiplier *= 3,
            SnailSymbol::RBracket => multiplier /= 2,
            SnailSymbol::Num(num) => output += multiplier * *num as usize,
            SnailSymbol::Comma => multiplier = (multiplier / 3) * 2        
        }
    }
    output
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
        let mut current_number = self.numbers[0].clone();
        let mut temp = vec![];

        for next_number in &self.numbers[1..] {
            std::mem::swap(&mut current_number, &mut temp);
            current_number.clear();
            add_snailnum(&temp, next_number, &mut current_number);
            reduce(&mut current_number);            
        }

        magnitude(&current_number)
    }

    fn part2(&mut self) -> usize {
        let mut temp = vec![];
        self.numbers
            .iter().enumerate().map(|(lidx, lhs)| {
                self.numbers.iter().enumerate().filter_map(move |(ridx, rhs)| {
                    if lidx != ridx {
                        Some((lhs, rhs))
                    } else {
                        None
                    }
                })
            }).flatten().map(|(lhs, rhs)| {
                temp.clear();
                add_snailnum(lhs, rhs, &mut temp);
                reduce(&mut temp);
                magnitude(&temp)
            }).max().unwrap()            
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() {
        let number = parse_snail("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        println!("{}", number.iter().map(|s| s.to_char()).collect::<String>());
        let result: SnailfishNumber = vec![
            SnailSymbol::LBracket, SnailSymbol::LBracket, SnailSymbol::LBracket,
            SnailSymbol::Num(9), SnailSymbol::Comma, SnailSymbol::LBracket,
            SnailSymbol::Num(3), SnailSymbol::Comma, SnailSymbol::Num(8), 
            SnailSymbol::RBracket, SnailSymbol::RBracket, SnailSymbol::Comma,
            SnailSymbol::LBracket, SnailSymbol::LBracket, SnailSymbol::Num(0),
            SnailSymbol::Comma, SnailSymbol::Num(9), SnailSymbol::RBracket,
            SnailSymbol::Comma, SnailSymbol::Num(6), SnailSymbol::RBracket,
            SnailSymbol::RBracket, SnailSymbol::Comma, SnailSymbol::LBracket,
            SnailSymbol::LBracket, SnailSymbol::LBracket, SnailSymbol::Num(3),
            SnailSymbol::Comma, SnailSymbol::Num(7), SnailSymbol::RBracket,
            SnailSymbol::Comma, SnailSymbol::LBracket, SnailSymbol::Num(4),
            SnailSymbol::Comma, SnailSymbol::Num(9), SnailSymbol::RBracket,
            SnailSymbol::RBracket, SnailSymbol::Comma, SnailSymbol::Num(3),
            SnailSymbol::RBracket, SnailSymbol::RBracket,
        ];

        
        assert_eq!(number, result);

        let number = parse_snail_double_digit("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        assert_eq!(number, result);
    }

    #[test]
    fn test_explode() {
        let mut number = parse_snail("[[[[[9,8],1],2],3],4]");
        let result = parse_snail("[[[[0,9],2],3],4]");
        explode(&mut number);
        assert_eq!(number , result);

        let mut number = parse_snail("[7,[6,[5,[4,[3,2]]]]]");
        let result = parse_snail("[7,[6,[5,[7,0]]]]");
        explode(&mut number);
        assert_eq!(number, result);

        let mut number = parse_snail("[[6,[5,[4,[3,2]]]],1]");
        let result = parse_snail("[[6,[5,[7,0]]],3]");
        explode(&mut number);
        assert_eq!(number, result);

        let mut number = parse_snail("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let result = parse_snail("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        explode(&mut number);
        assert_eq!(number, result);

        let mut number = parse_snail("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let result = parse_snail("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        explode(&mut number);
        assert_eq!(number, result);
    }

    #[test]
    fn test_add() {
        let lhs = parse_snail("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let rhs = parse_snail("[1,1]");
        let result = parse_snail("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let mut output = vec![];
        add_snailnum(&lhs, &rhs, &mut output);
        assert_eq!(output, result);
    }

    #[test]
    fn test_reduce_steps() {
        let mut number = parse_snail("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let flow = explode(&mut number);
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(number, parse_snail("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"));

        let flow = explode(&mut number);
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(number, parse_snail_double_digit("[[[[0,7],4],[15,[0,13]]],[1,1]]"));

        {
            let flow = explode(&mut number);
            assert_eq!(flow, ControlFlow::Break(()));
            assert_eq!(number, parse_snail_double_digit("[[[[0,7],4],[15,[0,13]]],[1,1]]"));
        }

        let flow = split(&mut number);
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(number, parse_snail_double_digit("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));

        {
            let flow = explode(&mut number);
            assert_eq!(flow, ControlFlow::Break(()));
            assert_eq!(number, parse_snail_double_digit("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"));
        }
        
        let flow = split(&mut number);
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(number, parse_snail("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));

        let flow = explode(&mut number);
        assert_eq!(flow, ControlFlow::Continue(()));
        assert_eq!(number, parse_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        {
            let flow = explode(&mut number);
            assert_eq!(flow, ControlFlow::Break(()));
            assert_eq!(number, parse_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

            let flow = split(&mut number);
            assert_eq!(flow, ControlFlow::Break(()));
            assert_eq!(number, parse_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        }
    }

    #[test]
    fn test_reduce() {
        let mut number = parse_snail("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        let result = parse_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        reduce(&mut number);
        assert_eq!(number, result);
    }

    #[test] 
    fn test_magnitude() {
        let number = parse_snail("[[1,2],[[3,4],5]]");
        assert_eq!(magnitude(&number), 143);

        let number = parse_snail("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(magnitude(&number), 1384);

        let number = parse_snail("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(magnitude(&number), 445);

        let number = parse_snail("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(magnitude(&number), 791);

        let number = parse_snail("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(magnitude(&number), 1137);

        let number = parse_snail("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(magnitude(&number), 3488);
    }
}

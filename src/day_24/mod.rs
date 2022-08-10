#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
enum Rhs {
    Value(i64),
    Variable(usize), // Position in array [w, x, y, z],
    None
}

type Args = (usize, Rhs);


/** 
 * Corrects the input sequence so that it passes test.
 * 
 * The input consists of 14 processing blocks - each for one of the inputs.
 * Each of them has the same structure and is parametherized by three
 * variables, let's call them "a", "b" and "c". Written in pseudocode and using "w"
 * for the input value each block performs the following comutation:
 * 
 *      x = (z % 26 + b) != w
 *      z /= a
 *      z *= 25 * x + 1
 *      z += (w + c) * x
 * 
 * This can be re-written using an if-block, which eliminates the x-register:
 * 
 *      if z % 26 + b != w {
 *          z /= a
 *          z *= 26
 *          z += w + c
 *      } else {
 *          z /= a
 *      }
 * 
 * We figure out from input, that "a" can only be one of two values: either 1 or 26.
 * This leads us to observe that all computations are manipulations of digits of the 
 * z-register written in base 26. So it's natural to define "a = 26 ^ shf", so that 
 * "shf" will be either 0 or 1. We can use binary operators to denote operations in 
 * base 26 as follows:
 * 
 *      z * 26 = z << 1
 *      z / 26 = z >> 1
 *      z % 26 = z & 1
 * 
 * With this we can write the program as follows: 
 * 
 *      if z & 1 + b != w {
 *          z = z >> shf
 *          z = z << 1 
 *          z = z + (w + a)
 *      } else {
 *          z = z >> shf     
 *      }
 * 
 * We can also write the bitwise operations as follows: 
 * 
 *      z & 1 = z.last_bit
 *      z >> 1 = z.pop()
 *      (z << 1) & q = z.push(q)
 *      ((z >> 1) << 1) & q = z.pop_push(q)
 * 
 * where pop/push refer to that bit stack of z in base 26 with the last bit 
 * on top. Therefore, z.pop() removes the last bit, z.push(q) appends 
 * the bit "q", and z.pop_push(q) replaces the last bit by "q".
 * 
 * Given that "shf" can only be 0 or 1 we get the following two cases:
 * 
 *      if shf == 0 {
 *          if z.last_bit + b != w {
 *              z.push(w + c)
 *          }
 *      } elif shf == 1 {
 *          if z.last_bit + b != w {
 *              z.pop_push(w + c)
 *          } else {
 *              z.pop()
 *          }
 *      }
 * Accordin to the puzzle input (our input) in all cases where shf == 0
 * it's true that b > 9. Given that 1 <= w <= 9 the check (if z.last_bit + b != w)
 * will therefore always be true. This gives:
 * 
 *      if shf == 0 {
 *          z.push(w + c)
 *      } elif shf == 1 {
 *          if z.last_bit + b == w {
 *              z.pop()
 *          } else {
 *              z.pop_push(w + c)
 *          }
 *      }
 * 
 * We can summarize in words. View z as a stack of bits in base 26. Start with
 * an empty stack. Whenever shf == 0 (a == 1) push (w + c) on the stack. If,
 * however, shf == 1, consider the last bit on the stack. If it's equal to (w - b),
 * then remove it, otherwise replace it by (w + c).
 * 
 * We also observe from the puzzle input that among the 14 instruction blocks for
 * each of the nputs there are exactly 7 cases with shf == 0 and 7 with shf == 1.
 * Given that for shf == 0 something is always added to the stack, our goal is to 
 * arrange the input so that for shf == 1 it's always popped from stack, so that at
 * the end of the program we end up with an empty bit stack, which means that z == 0,
 * which makes the input pass the test.
 * 
 * To arrang this start with an arbitrary array of 14 inputs denoted by [w0, w1, ... w13].
 * If the first two instructions blocks have shf_0 == 0 and shf_1 == 0 then after the
 * first two inputs two bits will have been pushed to the stack:
 * 
 *      z_stack = [w0 + c0, w1 + c1]
 * 
 * If then shf_2 == 1 we want to set w2 so that the last bit is popped. 
 * The last bit is popped if:
 * 
 *      z.last_bit + b2 == w2
 *   => w1 + c1 + b2 == w2
 * 
 * So we set (w2 = w1 + c1 + b2). It can now occur that the condition 1 <= w2 <= 9 is
 * violated. In tis case we can add an arbitrary value to w2 to restore this condition.
 * We will need to add the same value to w1 too in order to maintain the previous 
 * equality. We need to be careful that after these adjustments we also maintain 
 * 1 <= w1 <= . The least we can do is for cases where w2 < 1 to choose the value so 
 * that w2 = 1 and for cases with w2 > 9 to choose the value so that w2 = 9. If this 
 * still doesn't work for w1, then no other value will work for both either.
 * 
 * This strategy can be used to take any with input sequence and correct it so that
 * it passes the test. So for part1 we'll want to start with the highest possible 
 * input (99_999_999_999_999), ad for part 2 with the lowest (11_111_111_111_111).
 * 
 * Programmatically, to correct the input we go through code subroutines that handle
 * each of the inputs and extract the (a or shf, b, c) parameters. If "shf" == 0 
 * we remember the "c" parameter by pushing it on a stack, and we also remember which 
 * input it corresponds to. If shf == 1 we pop the last "c" from the stack and use it 
 * to compute the correct input.
**/
fn correct_model_number(wish_input: [i64; 14], abcs: &[[i64; 3]; 14]) -> [i64; 14] {
    let mut number = wish_input.clone();
    let mut stack: Vec<(usize, i64)> = vec![];
    for (i, [a, b, c]) in abcs.iter().enumerate() {
        if *a == 1 {
            stack.push((i, *c));
        } else if *a == 26 {
            let (j, jc) = stack.pop().unwrap();
            number[i] = number[j] + jc + b;
            if number[i] > 9 {
                number[j] = number[j] - (number[i] - 9);
                number[i] = 9;
            } 
            if number[i] < 1 {
                number[j] = number[j] + (1 - number[i]);
                number[i] = 1;
            }
        }
    }
    number
} 

pub struct ArithemticLogicUnit {
    operations: Vec<(Operation, Args)>,
}

impl crate::Advent for ArithemticLogicUnit {
    fn new(data: &str) -> ArithemticLogicUnit {
        let operations: Vec<(Operation, Args)> = data.lines().map(|l| {
            let mut parts = l.split(" ");
            let operation = Operation::from_str(parts.next().unwrap());
            let lhs_char: &str = parts.next().unwrap();
            let lhs: usize = match lhs_char.chars().next().unwrap() {
                'w' => 0,
                'x' => 1,
                'y' => 2,
                'z' => 3,
                c => panic!("Invalid variable name {}", c)
            };

            let rhs: Rhs = match parts.next() {
                None => Rhs::None,
                Some(s) => {
                    let res: Result<i64, _> = s.parse();
                    if let Ok(val) = res {
                        Rhs::Value(val)
                    } else {
                        let char_index = match s.chars().next().unwrap() {
                            'w' => 0,
                            'x' => 1,
                            'y' => 2,
                            'z' => 3,
                            c => panic!("Invalid variable name {}", c)
                        };
                        Rhs::Variable(char_index)
                    }
                }
            };           
            (operation, (lhs, rhs))            
        }).collect();
        ArithemticLogicUnit { operations }   
    }

    fn part1(&mut self) -> usize {          
        let chunks = self.operation_chunks();  
        let abcs = extract_abcs(chunks);
        let number_arr = correct_model_number([9; 14], &abcs);
        let wxyz = self.calculate(number_arr);
        if wxyz[3] != 0 {
            panic!("Part 1 was impossible to calculate.");
        }
        let number = number_arr.iter().fold(0, |acc, digit| acc * 10 + digit) as usize;
        number
    }

    fn part2(&mut self) -> usize {
        let chunks = self.operation_chunks();  
        let abcs = extract_abcs(chunks);
        let number_arr = correct_model_number([1; 14], &abcs);
        let wxyz = self.calculate(number_arr);
        if wxyz[3] != 0 {
            panic!("Part 2 was impossible to calculate.");
        }
        let number = number_arr.iter().fold(0, |acc, digit| acc * 10 + digit) as usize;
        number
    }
}

/// Returns the three values for each chunk that are changing troought the chunks
fn extract_abcs(chunks: Vec<Vec<(Operation, (usize, Rhs))>>) -> [[i64; 3]; 14] {
    let abcs: [[i64; 3]; 14] = chunks.iter().map(|chunk| {
        let abc: [i64; 3] = chunk.iter().enumerate().filter(|(i, _)| {
            if [4, 5, 15].contains(i) {
                true
            } else {
                false
            }
        }).map(|(_i, (_op, (_lhs, rhs)))| {
            match rhs {
                Rhs::Value(x) => *x,
                _ => panic!("Didn't get value!")
            }
        }).collect::<Vec<i64>>().try_into().unwrap();
        abc
    }).collect::<Vec<[i64; 3]>>().try_into().unwrap();
    abcs
}

impl ArithemticLogicUnit {
    // fn display_differences(&mut self) {
    //     let mut chunks = self.operation_chunks().clone();
    //     let mut differences: Vec<(usize, (Operation, Args))> = vec![];
    //     while let Some((reference_chunk, remaining_chunks)) = chunks.split_first() {
    //         for c in remaining_chunks {
    //             for (i, (el, rel)) in c.iter().zip(reference_chunk).enumerate() {
    //                 if *el != *rel {
    //                     let new_diff = (i, rel.clone());
    //                     if !differences.contains(&new_diff) {
    //                         differences.push(new_diff);
    //                     }
    //                 }
    //             }
    //         }
    //         chunks = remaining_chunks.to_vec();
    //     }
    //     for (i, d) in differences.iter().sorted_by(|a, b| a.0.cmp(&b.0)) {
    //         println!("{} -> {:?}", i, d);
    //     }
    // }

    fn calculate(&mut self, number: [i64; 14]) -> [i64; 4] {
        let mut wxyz = [0; 4];
        for ((op, (lhs, rhs)), digit) in self.operations.iter().zip(number) {
            match op {
                Operation::Inp => {
                    wxyz[0] = digit;
                },
                _ => {
                    let value = match rhs {
                        Rhs::Value(v) => *v,
                        Rhs::Variable(var_index) => wxyz[*var_index],
                        Rhs::None => unreachable!()
                    };
                    op.calculate(&mut wxyz[*lhs], value);
                }
            }
        }

        wxyz
    }

    fn operation_chunks(&self) -> Vec<Vec<(Operation, Args)>> {
        let mut operation_iter = self.operations.clone().into_iter().peekable();

        let mut chunks: Vec<Vec<(Operation, Args)>> = vec![];
        let mut chunk: Vec<_> = vec![];
        let mut current = operation_iter.next();

        while current.is_some() {
            let current_el = current.take().unwrap();
            let next_el = operation_iter.peek();
            let end_of_chunk = if let Some((op, _args)) = next_el {
                match op {
                    Operation::Inp  => {
                        true
                    }, 
                    _ => false
                }            
            } else {
                true
            };
            chunk.push(current_el);

            if end_of_chunk {
                chunks.push(chunk.clone());
                chunk.clear();
            }

            current = operation_iter.next();
        }    
        chunks
    }
}
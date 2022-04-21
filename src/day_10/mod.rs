pub struct SyntaxScoring {
    lines: Vec<Vec<char>>
}

const OPENING_CHARS: [char; 4] = ['(', '[', '{', '<'];
// const CLOSING_CHARS: [char; 4] = [')', ']', '{', '>'];

fn match_chars(opening: &char, closing: &char) -> bool {
    match (opening, closing) {
        ('(', ')') | 
        ('[', ']') |
        ('{', '}') |
        ('<', '>' ) => true,
        _ => false
    }
}

fn get_closing_char(opening: &char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        c => panic!("Invalid opening char: {}", c)
    }
}

fn illegal_char_points(closing: &char) -> usize {
    match closing {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid closing char: {}", closing)
    }
}

fn missing_char_points(missing_chars: &Vec<char>) -> usize {    
    let mut points: usize = 0;

    missing_chars.iter().for_each(|c| {
        points *= 5;
        let char_points = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            x => panic!("Invalid closing char: {}", x)
        };
        points += char_points
    });

    points
}

impl SyntaxScoring {
    fn check_corrupt<'a> (&self, line: &'a Vec<char>) -> Result<Vec<&'a char>, &'a char> {
        let mut currently_open: Vec<&char> = Vec::new();
        for c in line.iter() {
            if OPENING_CHARS.contains(c) {
                currently_open.push(c);
            } else {
                let last_open = currently_open.remove(currently_open.len() - 1);
                if !match_chars(last_open, c) {
                    return Err(c);
                }
            }
        }           
        Ok(currently_open)
    }
}

impl crate::Advent for SyntaxScoring {
    fn new(data: &str) -> SyntaxScoring {
        let lines: Vec<Vec<char>> = data.lines().map(|line| {
            line.chars().collect::<Vec<char>>()
        }).collect();
        SyntaxScoring { lines }
    }

    fn part1(&self) -> usize {
        let mut illegal_chars: Vec<&char> = Vec::new();
        self.lines.iter().for_each(|line| {
            let res = self.check_corrupt(line);
            match res {
                Err(char) => illegal_chars.push(char),
                _ => ()
            }
        });
        
        illegal_chars.iter().map(|c| {
            illegal_char_points(c)
        }).sum()
    }

    fn part2(&self) -> usize {
        let missing_chars: Vec<Vec<char>> = self.lines.iter().filter_map(|line| {
            match self.check_corrupt(line) {
                Ok(currently_open) => Some(currently_open),
                Err(_) => None,
            }
        }).map(|currently_open| {
            currently_open.iter().rev().map(|c| get_closing_char(c)).collect()
        }).collect();

        let mut scores: Vec<usize> = missing_chars.iter().map(|mc| missing_char_points(mc)).collect();
        scores.sort();
        scores[scores.len() / 2]
    }
}
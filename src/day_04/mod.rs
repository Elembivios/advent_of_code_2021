pub struct GiantSquid {
    boards: Vec<Board>,
    draw_numbers: Vec<u8>
}

pub struct Board {
    data: [u8; 25]
}

impl Board {
    fn last_draw_index(&self, draw_numbers: &[u8]) -> Option<usize> {
        let mut rows: [u8; 5] = [0; 5];
        let mut cols: [u8; 5] = [0; 5];

        for (i, number) in draw_numbers.iter().enumerate() {
            if let Some(position) = self.data.iter().position(|v| v == number) {
                let row_index = position / 5;
                let col_index = position % 5;

                rows[row_index] += 1;
                cols[col_index] += 1;

                if rows[row_index] == 5 || cols[col_index] == 5 {
                    return Some(i);
                }
            }
        }

        None
    }

    fn winning_score(&self, draw_numbers: &[u8], last_draw_index: Option<usize>) -> usize {
        if let Some(index) = last_draw_index {
            let marked_numbers = draw_numbers.get(..=index).unwrap();
            let unmarked_sum: usize = self.data
                .iter()
                .filter(|v| !marked_numbers.contains(v))
                .map(|&v| v as usize)
                .sum();
            draw_numbers[index] as usize * unmarked_sum
        } else {
            0
        }
    }
}

impl crate::Advent for GiantSquid {
    fn new(data: &str) -> GiantSquid {
        let mut iter = data.lines();
        let draw_numbers: Vec<u8> = iter
            .next()
            .unwrap()
            .split(',')
            .flat_map(|c| c.parse())
            .collect();
        
        let mut boards = Vec::new();

        while iter.next().is_some() {
            let data = iter
                .by_ref()
                .take(5)
                .flat_map(|s| s.split_whitespace())
                .flat_map(|c| c.parse())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            boards.push(Board { data });
        }

        GiantSquid { draw_numbers, boards }
    }    

    fn part1(&mut self) -> usize {    
        let (board, last_draw_index) = self.boards
            .iter()
            .map(|b| (b, b.last_draw_index(&self.draw_numbers)))
            .min_by_key(|t| t.1)
            .unwrap();
        board.winning_score(&self.draw_numbers, last_draw_index)
    }

    fn part2(&mut self) -> usize {
        let (board, last_draw_index) = self.boards
            .iter()
            .map(|b| (b, b.last_draw_index(&self.draw_numbers)))
            .max_by_key(|t| t.1)
            .unwrap();
        board.winning_score(&self.draw_numbers, last_draw_index)
    }
}
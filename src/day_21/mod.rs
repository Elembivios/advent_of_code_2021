use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
struct Player {
    score: usize,
    position: usize
}


impl Player {
    fn new(position: usize) -> Self {
        Player { position, score: 0 }
    }

    fn update(&mut self, throw_sum: usize) {
        let remainder = (self.position + throw_sum) % 10;
        let new_position = if remainder == 0 {
            10 
        } else {
            remainder
        };
        self.score += new_position;
        self.position = new_position;
    }
}

type CacheType = HashMap<(Player, Player), (usize, usize)>;

#[inline]
fn quantum_play(cache: &mut CacheType, outcome_occurance: &[(usize, usize)], current_player: Player, waiting_player: Player) -> (usize, usize) {
    if current_player.score >= 21 {
        return (1, 0);
    }

    if waiting_player.score >= 21 {
        return (0, 1);
    }
    let cached_value = cache.get(&(current_player.clone(), waiting_player.clone()));
    if let Some(cached_value) = cached_value {
        return cached_value.clone();
    }
    let mut answer = (0, 0);
    for (outcome, occurance) in outcome_occurance {
        let mut current_player_copy = current_player.clone();
        current_player_copy.update(*outcome as usize);
        let (x1, y1) = quantum_play(
            cache, 
            outcome_occurance,
            waiting_player.clone(),
            current_player_copy
        );
        answer = (answer.0 + y1 * occurance, answer.1 + x1 * occurance)
    }
    cache.insert((current_player, waiting_player), answer);
    answer
}

struct Dice {
    throws: usize
}

impl Dice {
    fn throw(&mut self) -> usize {
        self.throws += 1;
        self.throws
    }
}

pub struct DiracDice {
    positions: Vec<usize>
}


impl crate::Advent for DiracDice {
    fn new(data: &str) -> DiracDice {
        let positions: Vec<usize> = data.lines().map(|l| {
            let c = l.chars().last().unwrap();
            c.to_digit(10).unwrap() as usize      
        }).collect();        
        DiracDice { positions }
    }

    fn part1(&mut self) -> usize {        
        let p1 = Player::new(self.positions[0]);
        let p2 = Player::new(self.positions[1]);
        let mut dice = Dice { throws: 0 };
        let goal: usize = 1000;
        let mut result: Option<(Player, Player)> = None;
        let (mut current_player, mut waiting_player) = (p1, p2);
        while result.is_none() {            
            let throw_sum: usize = (0..3).map(|_| {
                dice.throw()
            }).sum();
            current_player.update(throw_sum);
            if current_player.score >= goal {
                result = Some((current_player, waiting_player));
                break;
            }
            std::mem::swap(&mut current_player, &mut waiting_player);
        }
        
        let result = result.unwrap();
        result.1.score * dice.throws    
    }

    fn part2(&mut self) -> usize {
        let outcome_occurance: Vec<(usize, usize)> = (1..=3usize)
            .cartesian_product(1..=3usize)
            .cartesian_product(1..=3usize)
            .map(|((d1, d2), d3)| d1 + d2 + d3)
            .fold(HashMap::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            }).into_iter().collect_vec();
        let mut cache: CacheType = HashMap::new();
        let p1 = Player::new(self.positions[0]);
        let p2 = Player::new(self.positions[1]);
        let answer = quantum_play(&mut cache, &outcome_occurance, p1, p2);

        std::cmp::max(answer.0, answer.1)
    }
}
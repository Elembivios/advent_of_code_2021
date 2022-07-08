use std::fmt;
use std::io::{Error, ErrorKind};
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use hashbrown::HashMap;

type Space = Option<Amphipod>;

#[derive(Debug, PartialEq, Clone, Copy)]
struct State<const R: usize> {
    rooms: [[Space; R]; 4],
    hallway: [Space; 11]
}

impl<const R: usize> State<R> {
    fn from_str(data: &str) -> Self {
        let mut it = data.lines().skip(1);
        let hallway: [Space; 11] = it.next().unwrap().chars().filter_map(|c| {
            match c {
                '#' => None,
                ' ' => None,
                '.' => Some(None),
                _ => Some(Some(Amphipod::try_from(c).unwrap()))
            }
        }).collect::<Vec<_>>().try_into().unwrap();
        let rows: Vec<Vec<_>> = it.take(R).map(|l| {
            l.chars().filter_map(|c| {
                match c {
                    '#' => None,
                    ' ' => None,
                    '.' => Some(None),
                    _ => Some(Some(Amphipod::try_from(c).unwrap()))
                }
            }).collect()
        }).collect();

        let rooms: [[Space; R]; 4] = (0..4).map(|x| {
            let room: [Space; R] = rows.iter().map(|r| {
                r[x].clone()
            }).collect::<Vec<_>>().try_into().unwrap();
            room
        }).collect::<Vec<_>>().try_into().unwrap();

        State { rooms, hallway }
    }


    /// Encodes the state as an unsigned int.
    ///
    /// There's 5 states for each of the 27 spaces, that gives us 5^27 total combinations. It just
    /// happens that all those combinations neatly fit into a single u64, since 5^27 < 2^64.
    ///
    fn encode(&self) -> u64 {
        fn encode_space(space: Space) -> u64 {
            match space {
                None => 0,
                Some(amphipod) => amphipod as u64 + 1
            }
        }
        self.rooms.iter().flatten().rev()
            .chain(self.hallway.iter().rev())
            .map(|space| encode_space(*space))
            .fold(0, |encoded, encoded_space| encoded * 5 + encoded_space)
    }

    fn decode(mut encoded: u64) -> Self {
        fn decode_space(encoded_space: u64) -> Space {
            match encoded_space {
                0 => None,
                _ => Some(((encoded_space - 1) as usize).try_into().unwrap())
            }
        }
        let mut it = std::iter::from_fn(move || {
            let encoded_space = encoded % 5;
            encoded = encoded / 5;
            Some(decode_space(encoded_space))
        });

        Self { 
            hallway: [(); 11].map(|_| it.next().unwrap()),
            rooms: [(); 4].map(|_| [(); R].map(|_| it.next().unwrap()))            
        }
    }


    fn rows(&self) -> [[Space; 4]; R] {
        // Iterate rooms as rows 
        (0..R).map(|j| {
            (0..4).map(|i| {
                self.rooms[i][j]
            }).collect::<Vec<Space>>().try_into().unwrap()
        }).collect::<Vec<[Space; 4]>>().try_into().unwrap()
    }

    fn goal() -> Self {
        Self { rooms: [            
            [Some(Amphipod::A); R],
            [Some(Amphipod::B); R],
            [Some(Amphipod::C); R],
            [Some(Amphipod::D); R],

        ], hallway: [None; 11] }
    }

    /// Checks whether the room with the given index can be entered (by a matching amphipod).
    fn is_room_enterable(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| {
            match space {
                None => true,
                Some(amphipod) => amphipod.target_room_index() == room_index
            }
        })
    }

    /// Checks whether some ampthipods still have to exit the room with the given index
    fn is_room_exitable(&self, room_index: usize) -> bool {
        !self.is_room_enterable(room_index)
    }

    /// Maps from room index to hallway position of the space above the room.
    fn room_entrance_pos(&self, room_index: usize) -> usize {
        2 + room_index * 2
    }

    /// Checks whether a hiven hallway position is directly above one of the rooms
    #[inline]
    fn is_above_room(&self, x: usize) -> bool {
        x >= 2 && x < self.hallway.len() - 2 && x % 2 == 0
    }
    
    /// Check if an amphipod at start_x can freely move to target x.
    fn is_hallway_clear(&self, start_x: usize, target_x: usize) -> bool {
        let slice = match start_x.cmp(&target_x) {
            Ordering::Equal => { return true; },
            Ordering::Less => &self.hallway[(start_x + 1)..=target_x],
            Ordering::Greater => &self.hallway[target_x..start_x],
        };
        slice.iter().all(|space| space.is_none())
    }

    fn iter_empty_spaces(&self, hallway_pos: usize) -> impl Iterator<Item=usize> + '_ {
        let left_it = (0..hallway_pos).rev()
            .take_while(|x| self.hallway[*x].is_none());
        let right_it = ((hallway_pos + 1)..self.hallway.len())
            .take_while(|x| self.hallway[*x].is_none());
        left_it.chain(right_it)
    }

    /// Returns transitions where amphipods move out of a room into the hallway.
    fn room_to_hallway_transitions(&self) -> Vec<(State<R>, usize)> {
        self.rooms.iter().enumerate()
            .filter(|(room_index, _)| self.is_room_exitable(*room_index))
            .flat_map(|(room_index, room)| {
                // Find top_most amphipod
                // This always suceeds, because of the filter above
                let (room_depth, amphipod) = room.iter()
                    .enumerate().find_map(|(room_depth, space)| {
                        space.map(|amphipod| (room_depth, amphipod))
                    }).unwrap();

                let current_hallway_pos = self.room_entrance_pos(room_index);

                // Step in either direction as long as there is empty space.
                self.iter_empty_spaces(current_hallway_pos)
                    // Cannot move to a space directly above a room
                    .filter(|target_x| !self.is_above_room(*target_x))
                    .map(move |target_x| {
                        let steps = room_depth + 1 + current_hallway_pos.abs_diff(target_x);
                        let energy = steps * amphipod.energy();

                        let mut state = *self;
                        std::mem::swap(
                            &mut state.rooms[room_index][room_depth],
                            &mut state.hallway[target_x],
                        );
                        (state, energy)
                    })               
            }).collect()
    }


    /// Returns transitions where amphipods move from the hallway into therir target rooms.
    fn hallway_to_room_transitions(&self) -> Vec<(State<R>, usize)> {
        self.hallway.iter().enumerate()
            .filter_map(|(current_x, space)| {
                // Skip empty spaces
                space.map(|amphipod| (current_x, amphipod))
            })
            .filter_map(|(current_x, amphipod)| {
                let target_room_index = amphipod.target_room_index();

                if !self.is_room_enterable(target_room_index) {
                    // Target room still has other amphipods in it
                    return None;
                }

                let target_x = self.room_entrance_pos(target_room_index);

                if !self.is_hallway_clear(current_x, target_x) {
                    // Cannot move trough other anphipods
                    return None;
                }

                let target_room_depth = self.rooms[target_room_index].iter()
                    .rposition(|space| space.is_none())
                    .unwrap();
                
                let steps = target_room_depth + 1 + current_x.abs_diff(target_x);
                let energy = steps * amphipod.energy();

                let mut state = *self;
                std::mem::swap(
                    &mut state.rooms[target_room_index][target_room_depth],
                    &mut state.hallway[current_x]
                );

                Some((state, energy))
            }).collect()
    }


    /// Get all valid transitions from this state, together with their energy costs.
    fn transitions(&self) -> Vec<(State<R>, usize)> {
        let mut transitions = self.room_to_hallway_transitions();
        transitions.extend(self.hallway_to_room_transitions().into_iter());
        transitions
    }

    /// Heuristic function for the A* alhorithm. Returns a lower bound on the energy cost
    /// needed to reach the goal state from this state.
    fn h_score(&self) -> usize {
        let exit_cost = self.rooms.iter().enumerate()
            .flat_map(|(room_index, room)| {
                let current_hallway_pos = self.room_entrance_pos(room_index);

                // Amphipods that must move out of the current room,
                // either because they belong in another room, or 
                // because they have to get out of the way for an 
                // amphipod below.
                room.iter().enumerate().rev()
                    .filter_map(|(room_depth, space)| {
                        // Filter out empty spaces
                        space.map(|amphipod| (room_depth, amphipod))
                    })
                    .skip_while(move |(_, amphipod)| {
                        // Skip amphipods that don't need to move
                        amphipod.target_room_index() == room_index
                    })
                    .map(move |(room_depth, amphipod)| {
                        let target_room_index = amphipod.target_room_index();
                        let target_hallway_pos = self.room_entrance_pos(target_room_index);

                        // Minimum number of steps this amphipod must make 
                        // in the hallway. For amphipods no in the right 
                        // room, this is the number of steps to reach the
                        // target room. For amphipods that ARE in the right room, 
                        // but need to make space, this is 2 (since it neds to move
                        // aside and back again).
                        let hallway_steps = current_hallway_pos.abs_diff(target_hallway_pos).max(2);
                        let steps = room_depth + 1 + hallway_steps;

                        steps * amphipod.energy()
                    })
            })
            .sum::<usize>();

        // Energy cost of amphipods in the hallway moving to the space above their target room
        let hallway_move_cost = self.hallway.iter().enumerate()
            .filter_map(|(current_hallway_pos, space)| {
                space.map(|amphipod| (current_hallway_pos, amphipod))
            })
            .map(|(current_hallway_pos, amphipod)| {
                let target_room_index = amphipod.target_room_index();
                let target_hallway_pos = self.room_entrance_pos(target_room_index);
                let steps = current_hallway_pos.abs_diff(target_hallway_pos);

                steps * amphipod.energy()
            })
            .sum::<usize>();

        // Energy cost of amphipods entering their target room from the space above it
        let enter_cost = self.rooms.iter().enumerate()
            .flat_map(|(room_index, room)| {
                room.iter().enumerate().rev()
                    .skip_while(move |(_, space)| {
                        if let Some(amphipod) = space {
                            // Skip amphipods that don't need to move
                            amphipod.target_room_index() == room_index
                        } else {
                            false
                        }
                    })
                    .map(move |(room_depth, _)| {
                        let target_amphipod = Amphipod::from_room_index(room_index);
                        let steps = room_depth + 1;

                        steps * target_amphipod.energy()
                    })
            }).sum::<usize>();

        exit_cost + hallway_move_cost + enter_cost
    }


}

impl<const R: usize> fmt::Display for State<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{:#<13}\n", "")?;
        let hallway: String = self.hallway.clone().into_iter().map(|space| {
            match space {
                Some(a) => a.into(),
                None => '.'
            }
        }).collect();
        write!(f, "#{}#\n", hallway)?;
        for (i, row) in self.rows().iter().enumerate() {
            let values: Vec<char> = row.clone().into_iter().map(|space| {
                match space {
                    Some(a) => a.into(),
                    None => '.'
                }
            }).collect();            
            if i == 0 {
                write!(f, "###{}#{}#{}#{}###\n", values[0], values[1], values[2], values[3])?;
            } else {
                write!(f, "  #{}#{}#{}#{}#\n", values[0], values[1], values[2], values[3])?;
            }
        }
        write!(f, "  {:#<9}\n", "")
    }
}


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }

    fn target_room_index(&self) -> usize {
        *self as usize
    }

    fn from_room_index(room_index: usize) -> Self {
        assert!(room_index < 4);
        unsafe { std::mem::transmute(room_index as u8)}
    }
}

impl TryFrom<usize> for Amphipod {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Amphipod::A),
            1 => Ok(Amphipod::B),
            2 => Ok(Amphipod::C),
            3 => Ok(Amphipod::D),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData,
                format!("Amphipod cannot be constructed from integer '{}'", value)
            ))
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Amphipod::A),
            'B' => Ok(Amphipod::B),
            'C' => Ok(Amphipod::C),
            'D' => Ok(Amphipod::D),
            _ => Err(Self::Error::new(
                ErrorKind::InvalidData, 
                format!("Amphipod cannot be constructed from characted '{}'", value)
            ))
        }
    }    
}

impl Into<char> for Amphipod {
    fn into(self) -> char {
        match self {
            Amphipod::A => 'A',
            Amphipod::B => 'B',
            Amphipod::C => 'C',
            Amphipod::D => 'D',
        }
    }
}

#[derive(PartialEq, Eq)]
struct Entry {
    encoded_state: u64,
    f_score: usize
}

impl PartialOrd<Self> for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

fn find_min_score<const R: usize>(initial_state: State<R>) -> usize {
    // Basically A* search algorithm.
    let encoded_initial_state = initial_state.encode();
    let mut q = BinaryHeap::new();
    q.push(Entry {
        encoded_state: encoded_initial_state,
        f_score: 0
    });

    let mut g_score: HashMap<u64, usize> = HashMap::new();
    g_score.insert(encoded_initial_state, 0);

    let encoded_goal_state = State::<R>::goal().encode();

    while let Some(Entry { encoded_state, f_score }) = q.pop() {
        if encoded_state == encoded_goal_state {
            return f_score;
        }

        let current_state = State::<R>::decode(encoded_state);
        let current_g_score = g_score[&encoded_state];

        for (next_state, transition_cost) in current_state.transitions() {
            let encoded_next_state = next_state.encode();
            let tentative_g_score = current_g_score + transition_cost;
            if tentative_g_score < *g_score.get(&encoded_next_state).unwrap_or(&usize::MAX) {
                g_score.insert(encoded_next_state, tentative_g_score);
                q.push(Entry {
                    encoded_state: encoded_next_state,
                    f_score: tentative_g_score + next_state.h_score()
                });
            }
        }
    }

    unreachable!("Puzzle is unsolvable!");
}


pub struct Amphipods {
    original_state_01: State<2>,
    original_state_02: State<4>
}

impl crate::Advent for Amphipods {
    fn new(data: &str) -> Amphipods {
        let original_state_01 = State::from_str(data);
        let insert = "  #D#C#B#A#\n  #D#B#A#C#";
        let result: Vec<&str> = data.lines().take(3).chain(
            insert.lines()
        ).chain(
            data.lines().skip(3)
        ).collect();
        let original_state_02 = State::from_str(result.join("\n").as_str());
        // println!("State 01: {}", original_state_01);
        // println!("State 02: {}", original_state_02);
        Amphipods { original_state_01, original_state_02 }
    }

    fn part1(&mut self) -> usize {
        find_min_score(self.original_state_01)
    }
    
    fn part2(&mut self) -> usize {
        find_min_score(self.original_state_02)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_state_01() -> State<2> {
        State::from_str(
            "#############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########"           
        )
    }

    fn get_state_02() -> State<4> {
        State::from_str(
            "#############
            #...........#
            ###B#C#B#D###
              #B#C#C#A#
              #A#D#B#C#
              #A#D#C#A#
              #########"           
        )
    }

    #[test]
    fn encode_decode_equal() {
        let state = get_state_01();
        let encoded = state.encode();
        let decoded = State::decode(encoded);
        assert_eq!(state, decoded);

        let state = get_state_02();
        let encoded = state.encode();
        let decoded = State::decode(encoded);
        assert_eq!(state, decoded);
    }

    #[test]
    fn test_pos_above_room() {
        let state = get_state_02();
        let mut it = state.hallway.iter().enumerate().map(|(x, _)| {
            state.is_above_room(x)
        });
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(true));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(true));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(true));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(true));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), Some(false));
        assert_eq!(it.next(), None);
    }
}
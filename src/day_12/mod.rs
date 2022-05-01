use std::rc::{Rc, Weak};
use std::cell::RefCell;

use itertools::Itertools;

const MAX_CAVES: usize = 11;

pub struct PassagePassing{
    caves: Vec<Rc<Cave>>
}

#[derive(Debug, Clone)]
struct Cave {
    value: String,
    connected: RefCell<Vec<Weak<Cave>>>,
    small: bool,
    stack: Vec<Rc<Cave>>
}

impl Cave {
    fn new(value: String) -> Cave {
        let small: bool = value.chars().any(|c| c.is_ascii_lowercase());
        Cave { value, connected: RefCell::new(Vec::new()), small, stack: vec![] }
    }
}
impl PartialEq<Cave> for Cave {
    fn eq(&self, other: &Cave) -> bool {
        self.value == other.value
    }
}

impl PartialEq<String> for Cave {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}

impl PartialEq<Cave> for String {
    fn eq(&self, other: &Cave) -> bool {
        *self == other.value
    }
}

// impl PartialEq<String> for Cave {
//     fn eq(&self, other: &String) -> bool {
//         self.value == *other
//     }
// }

// impl PartialEq<Cave> for Cave {
//     fn eq(&self, other: &Cave) -> bool {
//         self.value == other.value
//     }
// }

// impl Eq for Cave {}k

// struct CaveTraversal<'a> {
//     stack: Vec<&'a Cave>
// }

// impl<'a> CaveTraversal<'a> {
//     pub fn new(root: &'a Cave) -> Self {
//         CaveTraversal { stack: vec![root] }
//     }
// }


impl Iterator for Cave {
    type Item = Rc<Cave>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            for neighbour in node.connected.borrow().clone() {
                if let Some(cave) = neighbour.upgrade() {
                    self.stack.push(Rc::clone(&cave));
                }
            }
            return Some(node);
        }
        return None;
    }
}

struct CaveTraversal {
    stack: Vec<Rc<Cave>>,
    checked: Vec<usize> // Number of checked per node
}

impl CaveTraversal {
    pub fn new(root:Rc<Cave>) -> Self {
        CaveTraversal { stack: vec![root], checked: vec![0] }
    }
}

impl Iterator for CaveTraversal {
    type Item = Vec<Rc<Cave>>;

    fn next(&mut self) -> Option<Self::Item> {
        let stack: &mut Vec<Rc<Cave>> = &mut self.stack;
        let mut current_checked = self.checked.last_mut().unwrap();
        let current_cave = {
            stack.last().unwrap()
        };
        let connected = current_cave.connected.borrow();
        let to_check: Vec<&Weak<Cave>> = connected.iter().skip(*current_checked).filter(|cave| {
            let neighbour = cave.upgrade().unwrap();
            if neighbour.small && self.stack.contains(&neighbour) {
                return false;                
            } else {
                return true;
            }
        }).collect();

        if let Some(next) = to_check.iter().next() {
            let next = next.upgrade().unwrap();
            stack.push(Rc::clone(&next));

            *current_checked += 1;
            return Some(self.stack.clone());
        } else {
            return None;
        }
    }
}

impl crate::Advent for PassagePassing {
    fn new(data: &str) -> PassagePassing {
        // Construct unique caves
        let mut caves: Vec<Rc<Cave>> = Vec::new();
        data.lines().flat_map(|line| {
            line.split("-").map(|c| {
                Rc::new(Cave::new(c.to_string()))
            }).collect::<Vec<Rc<Cave>>>()
        }).for_each(|cave| {
            if let None = caves.iter().find(|c| c.value == cave.value) {
                caves.push(cave);
            }
        });

        // Connect caves
        data.lines().for_each(|line| {
            if let Some((lhs_cave_str, rhs_cave_str)) = line.split_once("-") {                
                let lhs_cave = caves.iter().find(|c| c.value == lhs_cave_str.to_string()).unwrap();
                let rhs_cave = caves.iter().find(|c| c.value == rhs_cave_str.to_string()).unwrap();

                let lhs_connection = Rc::downgrade(lhs_cave);
                let rhs_connection = Rc::downgrade(rhs_cave);

                let mut lhs_connected = lhs_cave.connected.borrow_mut();
                let mut rhs_connected = rhs_cave.connected.borrow_mut();

                lhs_connected.push(rhs_connection);
                rhs_connected.push(lhs_connection);
            }
        });

        println!("Caves: {:?}", caves);
        PassagePassing { caves }
    }

    fn part1(&mut self) -> usize {
        // let start = Rc::clone(&self.caves[0]).as_ref();
        // for cave in start.iter() {
        //     println!("Cave: {:?}", cave);
        // }

        
        let mut previous_cave: Option<Rc<Cave>> = None;
        let mut current_cave = Rc::clone(&self.caves[0]);        
        let mut current_path: Vec<Rc<Cave>> = vec![Rc::clone(&current_cave)];
        let mut neighbours: Vec<&Weak<Cave>> = current_cave.connected.borrow().iter().filter(|c| {
            if let Some(previous_cave) = &previous_cave {
                return c.upgrade().unwrap().value != previous_cave.value;                
            } else {
                return true;
            }
        }).map(|c| c).collect();

        let mut num_paths = 0;

        while neighbours.len() != 0 {
            
        }        
    
        // let cave_system = CaveTraversal::new(Rc::clone(&self.caves[0]));
        // for caves in cave_system {
        //     println!("Path: {:?}", caves);
        // }

        3
    }

    fn part2(&mut self) -> usize {
        4
    }
}
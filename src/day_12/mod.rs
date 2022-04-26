use std::rc::{Rc, Weak};

use itertools::Itertools;

const MAX_CAVES: usize = 11;

pub struct PassagePassing{
    caves: Vec<Cave>
}

#[derive(Debug)]
struct Cave {
    value: String,
    connected: Vec<Rc<Cave>>
}

impl Cave {
    fn new(value: String) -> Cave {
        Cave { value, connected: Vec::new()}
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

// impl<'a> Iterator for CaveTraversal<'a> {
//     type Item =  &'a Cave;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(node) = self.stack.pop() {
//             for neighbour in node.connected {
//                 self.stack.push(&neighbour);
//             }
//             return Some(node);
//         }
//         return None;
//     }
// }

impl crate::Advent for PassagePassing {
    fn new(data: &str) -> PassagePassing {

        // Construct unique caves
        let caves: Vec<Cave> = data
            .lines()
            .flat_map(|line| {
                line.split("-").map(|c| Cave::new(c.to_string())).collect::<Vec<Cave>>()
            }).dedup().collect();

        let qwe = caves[0] == "start".to_string();

    
        // Connect caves
        data.lines()
            .for_each(|line| {
                if let Some((lhs_cave, rhs_cave)) = line.split_once("-") {
                    let mut lhs_cave = caves.iter().find(|cave| cave.value == lhs_cave.to_string()).unwrap();
                    let rhs_cave = caves.iter().find(|cave| cave.value == rhs_cave.to_string()).unwrap();

                    lhs_cave.connected.push(Rc::new(&rhs_cave.clone()));


                }
                line.split("-").for_each(|c| {
                    let cave = caves.iter().find(|cave| cave.value == c.to_string()).unwrap();

                    println!("Cave: {:?}", cave);
                });
            });

        

                // if let Some((lhs, rhs)) = line.split_once("-") {

                //     if caves.contains(&lhs.to_string()) {

                //     }


                    // let lhs_cave = Cave::new(lhs.to_string());
                    // let rhs_cave = Cave::new(rhs.to_string());

                    // let lhs_index: usize = if let Some(i) = caves.iter().position(|each| *each.value == lhs.to_string()) {
                    //     i
                    // } else {
                    //     caves.push(lhs_cave);
                    //     caves.len() - 1
                    // };

                    // let rhs_index: usize = if let Some(i) = caves.iter().position(|each| *each.value == rhs.to_string()) {
                    //     i 
                    // } else {
                    //     caves.push(rhs_cave);
                    //     caves.len() - 1
                    // };                                
                    
                    // let lhs_cave: &mut Cave = caves.get_mut(lhs_index).unwrap();
                    // let rhs_cave: &mut Cave = caves.get_mut(rhs_index).unwrap();

                    // lhs_cave.connected.push(Rc::new(*rhs_cave));
                    // rhs_cave.connected.push(Rc::new(*lhs_cave));

                    // caves[lhs_index].connected.push(Rc::new(caves[rhs_index]));

                    // {
                    //     let lhs_cave: &mut Cave = &mut caves[lhs_index];
                    //     let rhs_cave: Cave = caves[rhs_index];
                    //     lhs_cave.connected.push(Rc::new(rhs_cave));
                        

                    // }
        PassagePassing { caves }
    }

    fn part1(&mut self) -> usize {
        3
    }

    fn part2(&mut self) -> usize {
        4
    }
}
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct PassagePassing{
    caves: Vec<Rc<Cave>>
}

#[derive(Debug, Clone)]
struct Cave {
    value: String,
    connected: RefCell<Vec<Weak<Cave>>>,
    small: bool,
}

impl Cave {
    fn new(value: String) -> Cave {
        let small: bool = value.chars().any(|c| c.is_ascii_lowercase());
        Cave { value, connected: RefCell::new(Vec::new()), small }
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


struct CaveTraversal {
    stack: Vec<Rc<Cave>>,
    checked: Vec<usize> // Number of checked per node
}

impl CaveTraversal {
    pub fn new(root:Rc<Cave>) -> Self {
        CaveTraversal { stack: vec![root], checked: vec![0] }
    }

    // Returns the next unchecked neighbour from the last cave in the stack
    fn inner_next(&mut self) -> (Option<Rc<Cave>>, usize) {      
        let index = self.stack.len() - 1;
        
        let current_cave = &self.stack[index];
        let current_checked: usize = self.checked[index];
        let mut skipped: usize = 0;
        let connected = current_cave.connected.borrow();
        
        for cave in connected.iter().skip(current_checked) {
            let neighbour = cave.upgrade().unwrap();
            if neighbour.small && self.stack.contains(&neighbour) {
                skipped += 1;
            } else {
                return (Some(Rc::clone(&neighbour)), skipped);
            }
        }
        (None, skipped)
    }

    // Walks backward till it finds a cave that not all neighbours were checked
    fn move_to_unchecked(&mut self) -> () {
        let mut current_index = self.stack.len() - 1;
        while self.checked[current_index] >= self.stack[current_index].connected.borrow().len() {
            self.stack.pop();
            self.checked.pop();
            if current_index == 0 {
                break;
            }
            current_index -= 1;            
        }
    }
}

impl Iterator for CaveTraversal {
    type Item = Vec<Rc<Cave>>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.stack.len() != 0 {
            let (next, skipped) = self.inner_next();             
            *self.checked.last_mut().unwrap() += skipped + 1;
            if let Some(next) = next {                            
                // let stack_str: Vec<String> = self.stack.iter().map(|c| c.value.clone()).collect();            
                // println!("Next: {}, Skipped: {}, Stack: {:?}, Checked: {:?}", next.value, skipped, stack_str, self.checked);
                if next.value == "end" {
                    let mut res = self.stack.clone();
                    res.push(Rc::clone(&next));
                    self.move_to_unchecked();
                    return Some(res);                    
                } else {
                    self.stack.push(Rc::clone(&next));
                    self.checked.push(0);                    
                }                
            } else {        
                self.move_to_unchecked();
            }            
        }

        None
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

        // println!("Caves: {:?}", caves);        
        PassagePassing { caves }
    }

    fn part1(&mut self) -> usize {
        self.caves.iter().for_each(|cave| {
            let connected: Vec<String> = cave.connected.borrow().iter().map(|c| c.upgrade().unwrap().value.clone()).collect();
            println!("{} -> {:?}", cave.value, connected);
        });

        let mut paths: Vec<Vec<Rc<Cave>>> = Vec::new();
        let start = Rc::clone(self.caves.iter().find(|cave| cave.value == "start").unwrap());
        let cave_traversal = CaveTraversal::new(start);

        for path in cave_traversal {
            paths.push(path);
        }

        paths.iter().for_each(|path| {
            let path_str: Vec<String> = path.iter().map(|cave| cave.value.clone()).collect();
            println!("Path: {:?}", path_str);
        });

        paths.len()
    }

    fn part2(&mut self) -> usize {
        4
    }
}
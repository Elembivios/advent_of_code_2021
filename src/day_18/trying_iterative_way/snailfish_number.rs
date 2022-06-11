use std::fmt;
use std::ops;

type ElPtr = Box<El>; 

#[derive(Debug, Clone)]
pub struct Node {
    lhs: ElPtr,
    rhs: ElPtr 
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.lhs == other.lhs && self.rhs == other.rhs
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.lhs.as_ref(), self.rhs.as_ref())
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum El {
    Nr(u32),
    Pr(Node)
}

impl El {
    pub fn from_str(s: &str) -> El {
        let mut opened: Vec<El> = vec![];
        s.chars().for_each(|c| {
            match c {
                '[' => {},
                ']' => {         
                    let rhs = opened.pop().unwrap();
                    let lhs = opened.pop().unwrap();

                    let new_node = Node {
                        lhs: lhs.into(),
                        rhs: rhs.into()
                    };
                    opened.push(El::Pr(new_node));         
                },
                ' ' => {},
                ',' => {},
                x => {
                    let x = x.to_digit(10).unwrap();
                    opened.push(El::Nr(x));
                }
            }
        });
        opened.pop().unwrap()
    }

    pub fn inorder_iter(&self) -> InorderTraversal {
        InorderTraversal::new(self)
    }

    pub fn inorder_iter_mut(&mut self) -> InorderTraversalMut {
        InorderTraversalMut::new(self)
    }

    pub fn split(&mut self) -> Option<()> {
        None
    }
}

impl fmt::Display for El {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            El::Nr(x) => { write!(f, "{}", x)},
            El::Pr(node) => { write!(f, "{}", node)}
        }
    }
}

impl ops::Add for El {
    type Output = El;
    fn add(self, rhs: El) -> El {
        El::Pr( Node {
            lhs: self.into(),
            rhs: rhs.into()
        })
    }    
}

pub struct InorderTraversal<'a> {
    current: Option<&'a El>,
    queue: Vec<(&'a El, usize)>,
    depth: usize
}

impl<'a> InorderTraversal<'a> {
    fn new(root: &'a El) -> InorderTraversal{
        InorderTraversal { current: Some(root), queue: vec![], depth: 0 }
    }
}

impl<'a> Iterator for InorderTraversal<'a> {
    type Item = (&'a El, usize);

    fn next(&mut self) -> Option<Self::Item> {    
        while !self.queue.is_empty() || self.current.is_some() {
            while let Some(el) = self.current {
                match el {
                    El::Pr(node) => {
                        self.depth += 1;
                        self.queue.push((el, self.depth));
                        self.current = Some(&node.lhs.as_ref());
                    },
                    El::Nr(_) => {                        
                        self.current = None;
                        return Some((el, self.depth));
                    }
                }            
            }
            if let Some((previous, previous_depth)) = self.queue.pop() {
                self.depth = previous_depth;
                match previous {
                    El::Pr(node) => { 
                        self.current = Some(&node.rhs.as_ref());
                    },
                    El::Nr(_) => { unreachable!() }
                }
            }
        }
        None
    }
}

pub struct InorderTraversalMut<'a> {
    current: &'a mut El,
    it: InorderTraversal<'a>
}

impl<'a> InorderTraversalMut<'a> {
    fn new(root: &'a mut El) -> InorderTraversalMut {
        InorderTraversalMut { current: root, it: root.inorder_iter() }
    }
}

impl<'a> Iterator for InorderTraversalMut<'a> {
    type Item = &mut El;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            El::Nr(_) => None,
            El::Pr(_) => {
                let next = self.it.next();
                if let Some((el, depth)) = next {
                    return Some(&mut *el)
                }
                None
            }
        }
    }
}
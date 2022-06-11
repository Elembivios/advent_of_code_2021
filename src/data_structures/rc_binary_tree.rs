// use std::collections::VecDeque;
// use std::rc::{Rc, RefCell};

// pub type NodePtr = Rc<RefCell<Node>>;

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Tree {
//     root: Option<NodePtr>,
// }

// impl Tree {
//     fn new() -> Self {
//         Tree { root: None }
//     }

//     fn level_iter(&self) -> LevelTraversal {
//         LevelTraversal::new(self.root.as_ref())
//     }

//     fn inorder_iter(&self) -> InorderTraversal {
//         InorderTraversal::new(self.root.as_ref())
//     }

//     fn insert(&mut self, value: i32) -> () {
//         self.insert_iterative(value);
//     }

//     fn insert_recursive(node: &mut NodePtr, value: i32) {
//         match node {
//             None => {
//                 *node = Node::new(value).into();                
//             }
//             Some(node) => {
//                 let mut node = node.borrow_mut();
//                 if value > node.value {
//                     insert_recursive(&mut node.right, value)
//                 } else if value < node.value {
//                     insert_recursive(&mut node.left, value)
//                 }
//             }
//         }
//     }

//     fn insert_iterative(&mut self, value: i32) {
//         if self.root.is_none() {
//             self.root = Node::new(value).into();
//             return;
//         }

//         let mut q: Vec<&mut NodePtr> = Vec::new();
//         let root = self.root.as_mut().unwrap();
//         q.push(root);

//         while let Some(node) = q.pop() {
//             let mut node = node.borrow_mut();
//             if value > node.value {
//                 let right = &mut node.right;
//                 match right {
//                     None => {
//                         *right = Node::new(value).into();
//                     },
//                     Some(n) => {
//                         q.push(n.clone());
//                     }
//                 }
//             } else if value < node.value {
//                 let left = &mut node.left;
//                 match left {
//                     None => {
//                         *left = Node::new(value).into();
//                     },
//                     Some(n) => {
//                         q.push(n.clone());
//                     }
//                 }
//             }
//         }

//     }

    
// }


// struct LevelTraversal<'a> {
//     current: Option<&'a NodePtr>,
//     queue: VecDeque<&'a NodePtr>
// }

// impl<'a> LevelTraversal<'a> {
//     fn new(node: Option<&'a NodePtr>) -> Self {
//         LevelTraversal { current: node, queue: VecDeque::new() }
//     }
// }

// impl<'a> Iterator for LevelTraversal<'a> {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         match (self.current, &mut self.queue) {
//             (None, q) if q.is_empty() => None,
//             (None, q) => {
//                 self.current = q.pop_front();
//                 self.next()
//             },
//             (Some(node), q) => {
//                 if let Some(ref left) = node.left {
//                     q.push_back(left);
//                 }
//                 if let Some(ref right) = node.right {
//                     q.push_back(right);
//                 }
//                 self.current = None;
//                 Some(node.value)
//             }
//         }
//     }
// }

// struct InorderTraversal<'a> {
//     current: Option<&'a NodePtr>,
//     queue: Vec<&'a NodePtr>,
// }


// impl<'a> InorderTraversal<'a> {
//     fn new(node: Option<&'a NodePtr>) -> Self {
//         InorderTraversal { current: node, queue: Vec::new() }
//     }
// }

// impl<'a> Iterator for InorderTraversal<'a> {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         match (self.current, &mut self.queue) {
//            (None, q) if q.is_empty() => None,
//            (None, q) => {
//                 let node = q.pop().unwrap();
//                 self.current = node.right.as_ref();
//                 Some(node.value)
//            },
//            (Some(node), q) => {
//                q.push(node);
//                self.current = node.left.as_ref();
//                self.next()
//            }
//         }
//     }
// }


// #[allow(dead_code)]
// #[derive(Debug)]
// struct Node {
//     value: i32,
//     left: Option<NodePtr>,
//     right: Option<NodePtr>
// }

// impl Node {
//     fn new(value: i32) -> Self {
//         Node {
//             value,
//             left: None,
//             right: None
//         }
//     }
// }

// impl From<Node> for Option<NodePtr> {
//     fn from(node: Node) -> Self {
//         Some(Rc::new(RefCell::new(node)))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn build_test_tree() -> Tree {
//         let mut tree = Tree::new();
//         tree.insert(8);
//         tree.insert(10);
//         tree.insert(3);
//         tree.insert(1);
//         tree.insert(6);
//         tree.insert(4);
//         tree.insert(7);
//         tree.insert(14);
//         tree.insert(13);

//         /*
//                  8
//                /   \
//               3    10
//              / \     \
//             1   6    14
//                / \   /
//               4   7 13
//         */

//         tree
//     }

//     #[test]
//     fn build_tree() {
//         let mut tree = Tree::new();
//         tree.insert(8);
//         tree.insert(10);
//         tree.insert(3);
//         tree.insert(1);
//         tree.insert(6);
//         tree.insert(4);
//         println!("Tree: {:?}", tree);
//         assert_eq!(tree.root.is_some(), true);        
//     }

//     #[test]
//     fn traverse_level_iter() {
//         let tree = build_test_tree();
//         let values: Vec<i32> = tree.level_iter().collect();
//         assert_eq!(values, vec![8, 3, 10, 1, 6, 14, 4, 7, 13]);
//     }

//     #[test]
//     fn traverse_inorder_iter() {
//         let tree = build_test_tree();
//         let values: Vec<i32> = tree.inorder_iter().collect();
//         assert_eq!(values, vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
//     }
    
// }
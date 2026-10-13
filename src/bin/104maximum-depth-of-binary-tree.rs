// Completed 10/13/2026
// Using DFS 
/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
struct Solution;
// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

use std::println;
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::{Ref, RefCell};
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        // Create base case
        if let Some(node) = root {
          // recurse through each node 
          let node = node.borrow_mut();
          depth += std::cmp::max(Self::max_depth(node.left.clone()), Self::max_depth(node.right.clone()));
          depth += 1;

        }
        depth
    }
}
// @lc code=end
fn main() {
  let mut node1 = TreeNode::new(3);
  let mut node2 = TreeNode::new(9);
  let mut node3 = TreeNode::new(20);
  let mut node4 = TreeNode::new(15);
  let mut node5 = TreeNode::new(7);
  node3.left = Some(Rc::new(RefCell::new(node4)));
  node3.right = Some(Rc::new(RefCell::new(node5)));
  node1.left = Some(Rc::new(RefCell::new(node2)));
  node1.right = Some(Rc::new(RefCell::new(node3)));
  println!("{}", Solution::max_depth(Some(Rc::new(RefCell::new(node1)))));
  
}


//completed 10/19/2026
/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
 */
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
            right: None,
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
//
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
use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let (Some(node_1), Some(node_2)) = (&p, &q) {
            if node_1.borrow().val != node_2.borrow().val {
                return false;
            }
            if !Self::is_same_tree(node_1.borrow().left.clone(), node_2.borrow().left.clone()) {
                return false;
            }
            if !Self::is_same_tree(node_1.borrow().right.clone(), node_2.borrow().right.clone()) {
                return false;
            }
            return true;
        }
        p.is_none() && q.is_none()
    }
}
// @lc code=end
fn main() {
    let mut node1 = TreeNode::new(1);
    let mut node2 = TreeNode::new(2);
    let mut node3 = TreeNode::new(3);

    node1.right = Some(Rc::new(RefCell::new(node3)));
    node1.left = Some(Rc::new(RefCell::new(node2)));
    let test_node = Some(Rc::new(RefCell::new(node1)));
    let copy_node = test_node.clone();
    println!("{}", Solution::is_same_tree(test_node, copy_node))
}

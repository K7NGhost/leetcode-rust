// Completed 10/26/2026
/*
 * @lc app=leetcode id=111 lang=rust
 *
 * [111] Minimum Depth of Binary Tree
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
use std::cell::RefCell;
use std::cmp;
use std::i32::MAX;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left = Self::min_depth(node.borrow().left.clone());
            let right = Self::min_depth(node.borrow().right.clone());

            if left == 0 {
                return right + 1;
            }
            if right == 0 {
                return left + 1;
            }
            return cmp::min(left, right) + 1;
        }
        0
    }
}
// @lc code=end
fn main() {}

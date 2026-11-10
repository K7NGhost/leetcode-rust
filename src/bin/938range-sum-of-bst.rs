/*
 * @lc app=leetcode id=938 lang=rust
 *
 * [938] Range Sum of BST
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
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        // traverse the bst if a number outside the range return zero but
        let mut sum = 0;
        if let Some(node) = root {
            let node = node.borrow();

            if node.val >= low && node.val <= high {
                sum += node.val;
            }
            if low <= node.val {
                sum += Self::range_sum_bst(node.left.clone(), low, high);
            }
            if node.val < high {
                sum += Self::range_sum_bst(node.right.clone(), low, high);
            }
            return sum;
        } else {
            return 0;
        }
    }
}
// @lc code=end

fn main() {}

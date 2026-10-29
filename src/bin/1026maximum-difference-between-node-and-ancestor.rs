// completed 10/27/2026
/*
 * @lc app=leetcode id=1026 lang=rust
 *
 * [1026] Maximum Difference Between Node and Ancestor
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // If we know the max number and min number we know the maximum number v
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_val: i32, min_val: i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();

                // check previous min and max for new min and max
                let min_val = min_val.min(node.val);
                let max_val = max_val.max(node.val);

                // determine the max number from the left side and right side and return that
                let left = dfs(node.left.clone(), max_val, min_val);
                let right = dfs(node.right.clone(), max_val, min_val);

                return left.max(right);
            }
            // when reaching the end determine the max number
            max_val - min_val
        }

        if let Some(node) = root {
            let val = node.borrow().val;
            dfs(Some(node), val, val)
        } else {
            0
        }
    }
}
// @lc code=end
fn main() {}

// completed 11/3/2026
/*
 * @lc app=leetcode id=515 lang=rust
 *
 * [515] Find Largest Value in Each Tree Row
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
use std::collections::VecDeque;
use std::i32::MIN;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut curr_max = 0;
        let mut ans = vec![];
        if let Some(node) = root {
            curr_max = node.borrow().val;
            queue.push_back(node);
        }
        while !queue.is_empty() {
            let size = queue.len();
            ans.push(curr_max);
            curr_max = MIN;
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                if let Some(left) = &node.left {
                    curr_max = curr_max.max(left.borrow().val);
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node.right {
                    curr_max = curr_max.max(right.borrow().val);
                    queue.push_back(right.clone());
                }
            }
        }
        ans
    }
}
// @lc code=end
fn main() {}

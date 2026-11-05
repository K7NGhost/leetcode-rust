// Completed 11/4/2026
/*
 * @lc app=leetcode id=103 lang=rust
 *
 * [103] Binary Tree Zigzag Level Order Traversal
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
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // use bfs with a switch to queue right instead of left
        let mut queue = VecDeque::new();
        let mut ans = vec![];
        let mut switch = true;
        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let size = queue.len();
            let mut depth_vec = VecDeque::new();

            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                if switch {
                    depth_vec.push_back(node.val);
                } else {
                    depth_vec.push_front(node.val);
                }
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
            if switch {
                switch = false;
            } else {
                switch = true;
            }
            let ans_vec = depth_vec.into();
            ans.push(ans_vec);
        }
        ans
    }
}
// @lc code=end
fn main() {}

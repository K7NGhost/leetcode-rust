// completed 10/28/2026
/*
 * @lc app=leetcode id=543 lang=rust
 *
 * [543] Diameter of Binary Tree
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
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // idea we recurse through both sides of the tree and then when returned to the root add the maxes together
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
            if let Some(node) = root {
                let node = node.borrow();

                let left = dfs(node.left.clone(), diameter);
                let right = dfs(node.right.clone(), diameter);

                *diameter = cmp::max(*diameter, left + right);

                return 1 + cmp::max(left, right);
            }
            0
        }

        let mut diameter = 0;
        dfs(root, &mut diameter);
        diameter
    }
}
// @lc code=end
fn main() {}

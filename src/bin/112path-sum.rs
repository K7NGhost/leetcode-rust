// Completed 10/15/2026
// idk if its a good implementation
/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut total_sum = target_sum;
        if let Some(node) = root {
            total_sum -= node.borrow().val;

            if Self::has_path_sum(node.borrow_mut().left.clone(), total_sum)
                || Self::has_path_sum(node.borrow_mut().right.clone(), total_sum)
            {
                return true;
            }
            if !node.borrow().left.is_some() && !node.borrow().right.is_some() {
                if total_sum == 0 {
                    return true;
                }
            }
        }

        false
    }
}
// @lc code=end
fn main() {
    let mut node1 = TreeNode::new(5);
    let mut node2 = TreeNode::new(4);
    let mut node3 = TreeNode::new(11);
    let mut node4 = TreeNode::new(7);
    let mut node5 = TreeNode::new(2);
    let mut node6 = TreeNode::new(8);
    let mut node7 = TreeNode::new(13);
    let mut node8 = TreeNode::new(4);
    let mut node9 = TreeNode::new(1);

    node3.left = Some(Rc::new(RefCell::new(node4)));
    node3.right = Some(Rc::new(RefCell::new(node5)));
    node2.left = Some(Rc::new(RefCell::new(node3)));
    node1.left = Some(Rc::new(RefCell::new(node2)));
    node8.right = Some(Rc::new(RefCell::new(node9)));
    node6.right = Some(Rc::new(RefCell::new(node8)));
    node6.left = Some(Rc::new(RefCell::new(node7)));
    node1.right = Some(Rc::new(RefCell::new(node6)));
    println!(
        "{}",
        Solution::has_path_sum(Some(Rc::new(RefCell::new(node1))), 22)
    );
}

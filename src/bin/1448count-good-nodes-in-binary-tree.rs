// Completed 10/18/2026
// dfs implementation not optimal
/*
 * @lc app=leetcode id=1448 lang=rust
 *
 * [1448] Count Good Nodes in Binary Tree
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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;

        ans += Self::dfs(root, vec![]);
        ans
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut prev_nums: Vec<i32>) -> i32 {
        let mut curr = 0;
        if let Some(node) = root {
            let node_ref = node.borrow_mut();
            prev_nums.push(node_ref.val);
            if prev_nums.iter().all(|&x| x <= node_ref.val) {
                curr += 1;
            }
            curr += Self::dfs(node_ref.left.clone(), prev_nums.clone());
            curr += Self::dfs(node_ref.right.clone(), prev_nums.clone());
        }

        return curr;
    }
}
// @lc code=end
fn main() {
    let mut node1 = TreeNode::new(3);
    let mut node2 = TreeNode::new(3);
    let mut node3 = TreeNode::new(4);
    let mut node4 = TreeNode::new(2);

    node2.left = Some(Rc::new(RefCell::new(node3)));
    node2.right = Some(Rc::new(RefCell::new(node4)));
    node1.left = Some(Rc::new(RefCell::new(node2)));
    println!(
        "{}",
        Solution::good_nodes(Some(Rc::new(RefCell::new(node1))))
    );
}

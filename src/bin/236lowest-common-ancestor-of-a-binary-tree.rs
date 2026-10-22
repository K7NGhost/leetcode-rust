// completed 10/21/2026
/*
 * @lc app=leetcode id=236 lang=rust
 *
 * [236] Lowest Common Ancestor of a Binary Tree
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;

        // check whether this is p or q
        if p.as_ref().is_some_and(|x| Rc::ptr_eq(&node, x))
            || q.as_ref().is_some_and(|x| Rc::ptr_eq(&node, x))
        {
            return Some(node);
        }

        // Search both subtrees
        let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());

        let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

        if left.is_some() && right.is_some() {
            return Some(node);
        }

        left.or(right)
    }
}
// @lc code=end
fn main() {
    let mut node1 = TreeNode::new(3);
    let mut node2 = TreeNode::new(5);
    let mut node3 = TreeNode::new(1);
    let mut node4 = TreeNode::new(6);
    let mut node5 = TreeNode::new(2);
    let mut node6 = TreeNode::new(0);
    let mut node7 = TreeNode::new(8);
    let mut node8 = TreeNode::new(7);
    let mut node9 = TreeNode::new(4);

    node5.right = Some(Rc::new(RefCell::new(node9)));
    node5.left = Some(Rc::new(RefCell::new(node8)));
    node2.right = Some(Rc::new(RefCell::new(node5)));
    node2.left = Some(Rc::new(RefCell::new(node4)));
    node3.right = Some(Rc::new(RefCell::new(node7)));
    node3.left = Some(Rc::new(RefCell::new(node6)));
    let test_node1 = Rc::new(RefCell::new(node2));
    let test_node2 = Rc::new(RefCell::new(node3));
    let copy_node1 = test_node1.clone();
    let copy_node2 = test_node2.clone();
    node1.right = Some(copy_node2);
    node1.left = Some(copy_node1);

    let ans = Solution::lowest_common_ancestor(
        Some(Rc::new(RefCell::new(node1))),
        Some(test_node1),
        Some(test_node2),
    );

    if let Some(node) = ans {
        println!("{}", node.borrow().val);
    }
}

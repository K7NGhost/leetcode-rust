// Completed 9/10/2026
use std::{env::VarError::NotUnicode, println};

/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
struct Solution;
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();

        while let Some(node) = current {
            while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                node.next = node.next.as_mut().unwrap().next.take();
            }

            current = node.next.as_mut();
        }

        head
    }
}
// @lc code=end
fn main() {
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(1);
    let mut n3 = ListNode::new(2);
    let mut n4 = ListNode::new(3);
    let mut n5 = ListNode::new(3);
    
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));
    let n1 = Some(Box::new(n1)); 
    let mut result_node = Solution::delete_duplicates(n1);
    while result_node.as_ref().is_some() {
        println!("{}", result_node.as_ref().unwrap().val);
        result_node = result_node.unwrap().next;
    }
}


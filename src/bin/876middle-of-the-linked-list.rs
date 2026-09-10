use std::println;

/*
 * @lc app=leetcode id=876 lang=rust
 *
 * [876] Middle of the Linked List
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // strat slow and faster pointer
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        slow.cloned()
    }
}
// @lc code=end

fn main() {
    let mut n1 = ListNode::new(1);
    let mut n2 = ListNode::new(2);
    let mut n3 = ListNode::new(3);
    let mut n4 = ListNode::new(4);
    let mut n5 = ListNode::new(5);
    let mut n6 = ListNode::new(6);
    n5.next = Some(Box::new(n6));
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));
    let mut test_node = Solution::middle_node(Some(Box::new(n1)));
    while test_node.is_some() {
        println!("{}", test_node.as_ref().unwrap().val);
        test_node = test_node.unwrap().next;
    }

}


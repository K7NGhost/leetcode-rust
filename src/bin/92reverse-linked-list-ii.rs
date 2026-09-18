/*
 * @lc app=leetcode id=92 lang=rust
 *
 * [92] Reverse Linked List II
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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        // create dummy node to point before
        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head,
        });
        // have a ref to the prev node before the reversal
        let mut prev = &mut dummy;
        for _ in 1..left {
            prev = prev.next.as_mut().unwrap();
        }
        // detach reversal part from left part
        let mut current = prev.next.take();

        // reverse up to right - left + 1
        let count = right - left + 1;

        // reverse exactly 'count' times
        let mut reversed = None; 
        for _ in 0..count {
            let mut node = current.take().unwrap();

            current = node.next.take();

            node.next = reversed;

            reversed = Some(node);
        }

        // find the tail of the reversed section

        // reconnect 
        dummy.next
    }
}
// @lc code=end

fn main() {

}
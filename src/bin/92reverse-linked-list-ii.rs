/*
 * @lc app=leetcode id=92 lang=rust
 *
 * [92] Reverse Linked List II
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        // create dummy node to point before
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        // have a ref to the prev node before the reversal
        // this is basically getting a mutable reference to the dummy variable
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
        let mut tail = reversed.as_mut().unwrap();

        for _ in 1..count {
            tail = tail.next.as_mut().unwrap();
        }

        // connect reversed section to the rest
        tail.next = current;

        // Connect previous section to reversed section
        prev.next = reversed;

        // reconnect
        dummy.next
    }
}
// @lc code=end

fn main() {
    let mut n1 = Some(Box::new(ListNode::new(1)));
    let mut n2 = Some(Box::new(ListNode::new(2)));
    let mut n3 = Some(Box::new(ListNode::new(3)));
    let mut n4 = Some(Box::new(ListNode::new(4)));
    let mut n5 = Some(Box::new(ListNode::new(5)));
    n4.as_mut().unwrap().next = n5;
    n3.as_mut().unwrap().next = n4;
    n2.as_mut().unwrap().next = n3;
    n1.as_mut().unwrap().next = n2;
    let mut test_node = Solution::reverse_between(n1, 2, 4);
    while test_node.is_some() {
        println!("{}", test_node.as_ref().unwrap().val);
        test_node = test_node.unwrap().next;
    }
}

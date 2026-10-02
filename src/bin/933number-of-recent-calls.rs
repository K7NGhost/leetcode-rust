// Completed 10/2/2026
// Queue implementation
// O(n) time complexity
/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 */

// @lc code=start

use std::collections::VecDeque;
struct RecentCounter {
    request_q: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        let request_q: VecDeque<i32> = VecDeque::new();
        RecentCounter { request_q }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(&front) = self.request_q.front() {
            if front < t - 3000 {
                self.request_q.pop_front();
            } else {
                break;
            }
        }
        self.request_q.push_back(t);
        return self.request_q.len() as i32;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end
fn main() {
    let mut test = RecentCounter::new();
    println!("{}", test.ping(1));
    println!("{}", test.ping(100));
    println!("{}", test.ping(3001));
    println!("{}", test.ping(3002));
}

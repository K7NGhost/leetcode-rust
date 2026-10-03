// Completed 10/3/2026
// Using queue implementation
// O(1) time complexity
//
/*
 * @lc app=leetcode id=346 lang=rust
 *
 * [346] Moving Average from Data Stream
 */

// @lc code=start
use std::collections::VecDeque;
struct MovingAverage {
    size: i32,
    nums: VecDeque<i32>,
    total: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage {
            size,
            nums: VecDeque::new(),
            total: 0,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.total += val;
        self.nums.push_back(val);
        if self.nums.len() > self.size as usize {
            if let Some(val) = self.nums.pop_front() {
                self.total -= val;
            }
        }
        let average: f64 = self.total as f64 / self.nums.len() as f64;
        average
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */
// @lc code=end
fn main() {
    let mut moving_average = MovingAverage::new(3);
    println!("{}", moving_average.next(1));
    println!("{}", moving_average.next(10));
    println!("{}", moving_average.next(3));
    println!("{}", moving_average.next(5));
}

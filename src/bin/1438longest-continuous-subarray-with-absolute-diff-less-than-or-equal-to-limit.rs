struct Solution;
/*
 * @lc app=leetcode id=1438 lang=rust
 *
 * [1438] Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
 */

// @lc code=start
use std::{collections::VecDeque, path::absolute};
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut ans = 0;
        for num in nums {
            queue.push_back(num);
            if queue.front().is_some() && !((queue.front().unwrap() - num).abs() <= 4) {
                queue.pop_front();
            }
        }

        ans = queue.len() as i32;
        ans
    }
}
// @lc code=end
fn main() {
    let test = vec![8, 2, 4, 7];
    let ans = Solution::longest_subarray(test, 4);
    println!("{}", ans);
}

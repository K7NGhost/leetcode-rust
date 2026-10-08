struct Solution;
/*
 * @lc app=leetcode id=1438 lang=rust
 *
 * [1438] Longest Continuous Subarray With Absolute Diff Less Than or Equal to Limit
 */

// @lc code=start
use std::{collections::VecDeque, path::absolute, vec};
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        // Tracks the minimum value at the front
        let mut increasing = VecDeque::new();
        // Tracks the maximum value at the front
        let mut decreasing = VecDeque::new();
        let mut left = 0;
        let mut ans = 0;

        for (right, num) in nums.iter().enumerate() {
            // Remove larger values since num is a better minimum candidate
            while increasing.back().is_some() && increasing.back().unwrap() > num {
                increasing.pop_back();
            }
            // Remove smaller values since num is a better maximum candidate
            while decreasing.back().is_some() && decreasing.back().unwrap() < num {
                decreasing.pop_back();
            }
            // Add the current number to both deques
            increasing.push_back(*num);
            decreasing.push_back(*num);

            // Shrink the window while max - min exceeds the limit
            while decreasing.front().is_some()
                && increasing.front().is_some()
                && decreasing.front().unwrap() - increasing.front().unwrap() > limit
            {
                // Remove the maximum if it's leaving the window
                if nums[left] == *decreasing.front().unwrap() {
                    decreasing.pop_front();
                }
                // Remove the minimum if it's leaving the window
                if nums[left] == *increasing.front().unwrap() {
                    increasing.pop_front();
                }
                // Move the left boundary forward to shrink the window
                left += 1;
            }

            // keep the longest valid window length found so far
            ans = ans.max(right - left + 1);
        }
        return ans as i32;
    }
}
// @lc code=end
fn main() {
    let test = vec![8, 2, 4, 7];
    let test2 = vec![4, 2, 2, 2, 4, 4, 2, 2];
    let ans = Solution::longest_subarray(test2, 0);
    println!("{}", ans);
}

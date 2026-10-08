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
        let mut increasing = VecDeque::new();
        let mut decreasing = VecDeque::new();
        let mut left = 0;
        let mut ans = 0;

        for (right, num) in nums.iter().enumerate() {
            while increasing.back().is_some() && increasing.back().unwrap() > num {
                increasing.pop_back();
            }
            while decreasing.back().is_some() && decreasing.back().unwrap() < num {
                decreasing.pop_back();
            }

            increasing.push_back(*num);
            decreasing.push_back(*num);

            while decreasing.front().is_some()
                && increasing.front().is_some()
                && decreasing.front().unwrap() - increasing.front().unwrap() > limit
            {
                if nums[left] == *decreasing.front().unwrap() {
                    decreasing.pop_front();
                }
                if nums[left] == *increasing.front().unwrap() {
                    increasing.pop_front();
                }

                left += 1;
            }
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

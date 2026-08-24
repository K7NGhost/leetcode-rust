// Completed 3/27/2026

use core::num;
use std::{path::Prefix, vec};

/*
 * @lc app=leetcode id=1413 lang=rust
 *
 * [1413] Minimum Value to Get Positive Step by Step Sum
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        // keep counter of sum
        // if ever less than 1 break loop
        // increase start value
        // prefix sum ds

        let mut start_value: i32 = 1;
        let mut flag: bool = true;
        let mut sum: i32 = 1;
        while flag {
            // start with first sum
            let mut prefix = vec![start_value + nums[0]];
            sum = prefix[0];
            if sum < 1 {
                sum = 0;
                start_value += 1;
                continue;
            }
            if nums.len() == 1 && nums[0] < 1 {
                start_value = nums[0].abs() + 1
            }

            for index in 1..nums.len() {
                sum = nums[index] + prefix[prefix.len() - 1];

                if sum < 1 {
                    sum = 0;
                    start_value += 1;
                    break;
                }
                prefix.push(sum);
            }
            if sum != 0 {
                flag = false;
            }
        }
        start_value
    }
}
// @lc code=end

fn main() {
    let mut test_arr = vec![-3, 2, -3, 4, 2];
    let mut test_arr2 = vec![-12];
    let mut test_arr3 = vec![-3, 6, 2, 5, 8, 6];
    let ans = Solution::min_start_value(test_arr3);
    println!("{}", ans);
}

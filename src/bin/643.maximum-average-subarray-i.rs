// Completed 3/21/2026
/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

use core::num;

struct Solution;

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut subarray = Vec::new();
        let mut total_sum = 0;
        let mut ans = f64::MIN;

        for i in 0..nums.len() {
            total_sum += nums[i];
            subarray.push(nums[i]);

            if subarray.len() > k {
                total_sum -= subarray.remove(0);
            }

            if subarray.len() == k {
                ans = ans.max(total_sum as f64 / k as f64);
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let nums = vec![1];
    let k = 1;
    let answer = Solution::find_max_average(nums, k);
    println!("The answer is: {}", answer);
}

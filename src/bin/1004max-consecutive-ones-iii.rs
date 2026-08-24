// Completed 3/23/2026
// Sliding Window technique
/*
 * @lc app=leetcode id=1004 lang=rust
 *
 * [1004] Max Consecutive Ones III
 */
struct Solution;
// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        // Use sliding window
        let mut left = 0;
        let mut cur = 0;
        let mut ans = 0;
        // keep expanding until the k contstraing is reached then contract
        for right in 0..nums.len() {
            if nums[right] == 0 {
                cur += 1;
            }

            while cur > k {
                if nums[left] == 0 {
                    cur -= 1;
                }

                left += 1;
            }

            ans = max(ans, right - left + 1)
        }

        ans as i32
    }
}
// @lc code=end

fn main() {
    let mut test_arr = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let test_ans = Solution::longest_ones(test_arr, 3);
    println!("Answer is {}", test_ans);
}

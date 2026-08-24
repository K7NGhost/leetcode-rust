// Completed 4/10/2026
/*
 * @lc app=leetcode id=1480 lang=rust
 *
 * [1480] Running Sum of 1d Array
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        // This is an implementation of prefix ds
        let mut ans_arr: Vec<i32> = vec![nums[0]];

        for index in 1..nums.len() {
            ans_arr.push(nums[index] + ans_arr[ans_arr.len() - 1]);
        }
        ans_arr
    }
}
// @lc code=end

fn main() {
    let test_arr = vec![1, 2, 3, 4];
    let ans_arr = Solution::running_sum(test_arr);
    for num in ans_arr {
        print!("{}, ", num);
    }
}

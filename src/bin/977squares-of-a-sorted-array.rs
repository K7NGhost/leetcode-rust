// Completed 3/21/2026
/*
 * @lc app=leetcode id=977 lang=rust
 *
 * [977] Squares of a Sorted Array
 */

use std::slice::SliceIndex;

struct Solution;

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // Sort method
        // // Create an array of the nums in squares
        // let mut ans_arr = Vec::new();

        // for num in nums {
        //     ans_arr.push(num.pow(2));
        // }
        // // sort the array
        // ans_arr.sort();

        // ans_arr

        // Two pointer method

        // Create a new array with the same size of ori array
        let mut ans_arr: Vec<i32> = vec![0; nums.len()];
        let mut right_pointer: usize = nums.len() - 1;
        let mut left_pointer: usize = 0;
        // go through the array
        for index in (0..nums.len()).rev() {
            if nums[left_pointer].abs() > nums[right_pointer].abs() {
                ans_arr[index] = nums[left_pointer].pow(2);
                left_pointer += 1;
            } else {
                ans_arr[index] = nums[right_pointer].pow(2);
                right_pointer -= 1;
            }
            if left_pointer > right_pointer {
                break;
            }
        }
        // two pointers check if current position check whether its max

        // move left pointer along

        ans_arr
    }
}
// @lc code=end

fn main() {
    let test_arr = vec![-4, -1, 0, 3, 10];
    let ans_arr = Solution::sorted_squares(test_arr);
    for num in ans_arr {
        print!("{}, ", num);
    }
}

// Completed 3/21/2026
/*
 * @lc app=leetcode id=1133 lang=rust
 *
 * [1133] Largest Unique Number
 */

use std::{collections::HashMap, hash::Hash};
struct Solution;
// @lc code=start
impl Solution {
    pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
        // Create a hashmap keep frequency of each number
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut max_num: i32 = -1;

        //traverse array
        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        for (key, value) in counter.into_iter() {
            if key > max_num && value == 1 {
                max_num = key;
            }

            println!("{} {}", key, value);
            println!("The max num is {}", max_num);
        }
        return max_num;
    }
}
// @lc code=end

fn main() {
    let arr = vec![5, 7, 3, 9, 4, 9, 8, 3, 1];
    Solution::largest_unique_number(arr);
}

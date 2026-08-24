/*
 * @lc app=leetcode id=525 lang=rust
 *
 * [525] Contiguous Array
 */
struct Solution;
// @lc code=start
use std::{cmp::max, collections::HashMap};
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        // could be implemented with a queue have a window
        let mut dic: HashMap<i32, i32> = HashMap::new();
        dic.insert(0, -1);
        let mut max_len = 0;
        let mut count = 0;

        for i in 0..nums.len() {
            count += if nums[i] == 1 { 1 } else { -1 };
            if dic.contains_key(&count) {
                max_len = max(max_len, i as i32 - dic.get(&count).unwrap());
            } else {
                dic.insert(count, i as i32);
            }
        }
        return max_len;
    }
}
// @lc code=end

fn main() {}

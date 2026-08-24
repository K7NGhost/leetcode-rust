// Completed 4/13/2026
/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */
struct Solution;
// @lc code=start

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create hashmap will store the key of num and index as value
        let mut dic: HashMap<i32, i32> = HashMap::new();
        // get the complement target - num
        for i in 0..nums.len() {
            let curr_num = nums[i];
            let complement = target - curr_num;
            // if complement is in hashmap return current index and the complement
            if dic.contains_key(&complement) {
                return vec![i as i32, dic[&complement]];
            }
            dic.insert(curr_num, i as i32);
        }

        return vec![-1, -1];
    }
}
// @lc code=end

fn main() {
    let test_arr = vec![2, 7, 11, 15];
    let ans_arr = Solution::two_sum(test_arr, 9);
    for num in ans_arr {
        println!("{}", num);
    }
}

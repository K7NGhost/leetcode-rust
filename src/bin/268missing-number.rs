// Completed 4/14/2026
/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // Create a loop from 0 to nums.len store each num in nums in a hashset
        let mut set: HashSet<i32> = HashSet::new();
        // if the number isnt in the hashset return i 

        for num in &nums {
            set.insert(*num);
        }

        for i in 0..nums.len() {
            let casted_ptr = i as i32;
            if !set.contains(&casted_ptr) {
                return i as i32;
            }
        }
        return nums.len() as i32; 
    }
}
// @lc code=end

fn main() {
    let test_arr = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let test_arr2 = vec![0, 1];
    let test_ans = Solution::missing_number(test_arr2);
    println!("ans: {}", test_ans);
}


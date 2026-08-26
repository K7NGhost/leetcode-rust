// Completed 8/26/2026
// A mix between hashing and sliding window
/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */
struct Solution;
// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // sliding window technqiue
        let chars: Vec<char> = s.chars().collect();

        let mut set = HashSet::new();
        let mut left = 0;
        let mut max_len = 0;
        for right in 0..chars.len() {
            while set.contains(&chars[right]) {
                set.remove(&chars[left]);
                left += 1;
            }

            set.insert(chars[right]);

            max_len = max_len.max(right - left + 1);
        }
        max_len as i32

    }
}
// @lc code=end
fn main() {
    let s = String::from("abcabcbb");
    let s2: String = String::from("pwwkew");
    println!("{}", Solution::length_of_longest_substring(s));
}

// Completed 10/1/2026
// Stack implementation
// 0ms O(n)
use std::{println, vec};
struct Solution;
/*
 * @lc app=leetcode id=1544 lang=rust
 *
 * [1544] Make The String Great
 */

// @lc code=start
impl Solution {
    pub fn make_good(s: String) -> String {
        // Stack implementation
        let mut stack: Vec<char> = vec![];
        for c in s.chars() {
            if !stack.is_empty() && stack[stack.len() - 1].is_lowercase() && c.is_ascii_uppercase() && stack[stack.len() - 1].to_ascii_uppercase() == c {
                stack.pop();
                continue;
            }
            if !stack.is_empty() && stack[stack.len() - 1].is_uppercase() && c.is_ascii_lowercase() && stack[stack.len() - 1] == c.to_ascii_uppercase() {
                stack.pop();
                continue;
                
            }
            stack.push(c);
        }
        let ans: String = stack.iter().collect();
        ans
    }
}
// @lc code=end
fn main() {
    let test = String::from("abBAcC");
    let test2 = String::from("leEeetcode");
    let test3 = String::from("Pp");
    let test4 = String::from("kkdsFuqUfSDKK");
    let ans = Solution::make_good(test4);
    println!("{}", ans);
}

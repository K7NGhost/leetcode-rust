// Completed 9/29/2026
// Used stack implementation
// 0ms O(n)
struct Solution;
/*
 * @lc app=leetcode id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if !stack.is_empty() && stack[stack.len() - 1] == c {
                stack.pop();
                continue;
            }
            stack.push(c);
        }
        let ans = stack.iter().collect();
        ans
    }
}
// @lc code=end
fn main() {
    let test_string = String::from("abbaca");
    let test_string2 = String::from("azxxzy");
    let ans = Solution::remove_duplicates(test_string2);
    println!("{}", ans);
}

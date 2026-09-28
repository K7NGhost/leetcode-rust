// Completed 9/28/2026
// Stack implementation
// 0ms O(n) time complexity
struct Solution;
/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Stack implementation
        // put the opening bracket on stack if closing bracket pop
        let mut stack = vec![];
        for char in s.chars() {
            if !stack.is_empty() {
                if char == ')' && stack[stack.len() - 1] == '(' {
                    stack.pop();
                    continue;
                }
                if char == '}' && stack[stack.len() - 1] == '{' {
                    stack.pop();
                    continue;
                }
                if char == ']' && stack[stack.len() - 1] == '[' {
                    stack.pop();
                    continue;
                }
            }

            stack.push(char);
        }

        if !stack.is_empty() {
            return false;
        }
        true
    }
}
// @lc code=end
fn main() {
    let test_string = String::from("([])");
    let test_string2 = String::from("(]");
    let test_string3 = String::from("()");
    let test_string4 = String::from("]");
    let ans = Solution::is_valid(test_string4);
    println!("{}", ans);
}

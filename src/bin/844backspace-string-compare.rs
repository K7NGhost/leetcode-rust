// Completed 9/30/2026
// Stack implementation
// 0ms O(2n) = O(n)
struct Solution;
/*
 * @lc app=leetcode id=844 lang=rust
 *
 * [844] Backspace String Compare
 */

// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        // two stacks here
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        for c in s.chars() {
            if c == '#' {
                stack1.pop();
                continue;
            }
            stack1.push(c);
        }

        for c in t.chars() {
            if c == '#' {
                stack2.pop();
                continue;
            }
            stack2.push(c);
        }

        let string1: String = stack1.iter().collect();
        let string2: String = stack2.iter().collect();
        if string1 == string2 {
            return true;
        }
        false
    }
}
// @lc code=end

fn main() {
    let test = String::from("ab#c");
    let test2 = String::from("ad#c");
    let test3 = String::from("ab##");
    let test4 = String::from("c#d#");
    let test5 = String::from("y#fo##f");
    let test6 = String::from("y#f#o##f");
    let ans = Solution::backspace_compare(test5, test6);
    println!("{}", ans);
}

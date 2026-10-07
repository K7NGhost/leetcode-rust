struct Solution;
/*
 * @lc app=leetcode id=921 lang=rust
 *
 * [921] Minimum Add to Make Parentheses Valid
 */

// @lc code=start
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut balance = 0;
        let mut moves = 0;
        for c in s.chars() {
            if c == '(' {
                balance += 1;
            } else {
                if balance > 0 {
                    balance -= 1;
                } else {
                    moves += 1;
                }
            }
        }
        moves + balance
    }
}
// @lc code=end
fn main() {
    let test = String::from("(((");
    println!("{}", Solution::min_add_to_make_valid(test));
}

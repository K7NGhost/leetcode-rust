// Completed 4/13/2026
/*
 * @lc app=leetcode id=1832 lang=rust
 *
 * [1832] Check if the Sentence Is Pangram
 */
struct Solution;
// @lc code=start
use std::{collections::HashSet, sync::mpsc::SendError};
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // Create a hashset add the entire alphabet
        let alphabet_string = String::from("abcdefghijklmnopqrstuvwxyz");
        let mut alphabet_dic = HashSet::new();
        for c in sentence.chars() {
            alphabet_dic.insert(c);
        }
        // create a loop if the character isnt in the alphabetset than return false;
        for c in alphabet_string.chars() {
            if !alphabet_dic.contains(&c) {
                return false;
            }
        }
        true
    }
}
// @lc code=end

fn main() {
    let test_string = String::from("thequickbrownfoxjumpsoverthelazydog");
    let ans = Solution::check_if_pangram(test_string);
    println!("{}", ans);
}

// Completed 8/24/2026
// Hashmap
/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

struct Solution;
// @lc code=start
use std::{collections::HashMap, println};
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // this can be solved using a hashmap
        let mut dic: HashMap<char, i32> = HashMap::new();
        // keep count of each letter in magazine then check if that count exists in ransomnote
        for c in ransom_note.chars() {
            *dic.entry(c).or_insert(0) += 1;
            println!("{}", c);
        }

        for c in magazine.chars() {
            if dic.contains_key(&c) {
                *dic.entry(c).or_default() -= 1;
            }
        }

        for val in dic.values() {
            if *val > 0 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

fn main() {
    let ransom_note = String::from("aa");
    let magazine = String::from("aab");
    println!("{}", Solution::can_construct(ransom_note, magazine));
}

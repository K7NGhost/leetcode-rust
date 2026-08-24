/*
 * @lc app=leetcode id=1189 lang=rust
 *
 * [1189] Maximum Number of Balloons
 */

struct Solution;
// @lc code=start
use std::cmp::min;
use std::collections::HashMap;
use std::i32::MAX;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // Create a hashmap counter of 'balloon'
        // loop through the text if char is in the map += 1
        let mut dic: HashMap<char, i32> = HashMap::new();
        let main_text = String::from("balloon");
        for char in main_text.chars() {
            dic.entry(char).or_insert(0);
        }
        for char in text.chars() {
            if dic.contains_key(&char) {
                dic.entry(char).and_modify(|x| *x += 1);
            }
        }
        let mut ans = MAX;
        for (key, value) in dic.iter_mut() {
            if *key == 'l' || *key == 'o' {
                let count: i32 = *value / 2;
                ans = min(ans, count);
            } else {
                ans = min(ans, *value);
            }
        }
        ans
    }
}
// @lc code=end

fn main() {}

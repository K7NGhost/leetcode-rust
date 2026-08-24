// Completed 8/24/2026
// Hashmap
/*
 * @lc app=leetcode id=771 lang=rust
 *
 * [771] Jewels and Stones
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        // create hashmap
        let mut dic: HashMap<char, i32> = HashMap::new();
        let mut sum: i32 = 0;
        // use it as a counter
        for c in jewels.chars() {
            dic.entry(c).or_insert(0);
        }

        for c in stones.chars() {
            if dic.contains_key(&c) {
                dic.entry(c).and_modify(|x| {*x += 1});
            }
        }

        for v in dic.values() {
            sum += *v;
            println!("{}", *v);
        }
        sum
        // have a sum for total        
    }
}
// @lc code=end

fn main() {
    let jewels: String = String::from("aA");
    let stones: String = String::from("aAAbbbb");
    Solution::num_jewels_in_stones(jewels, stones);
}


// Completed 4/16/2026
/*
 * @lc app=leetcode id=2225 lang=rust
 *
 * [2225] Find Players With Zero or One Losses
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Create one hashmap of the losses for each value in vec
        // if the key has zero losses put it in the first list
        // if the key has only 1 loss value put it in the second list
        // make sure its sorted

        let mut losers_dic: HashMap<i32, i32> = HashMap::new();
        let mut winners_dic: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<Vec<i32>> = vec![vec![]; 2];

        for row in &matches {
            for i in 0..row.len() {
                let var2 = winners_dic.entry(row[0]).or_insert(0);
                *var2 += 1;
                let var = losers_dic.entry(row[row.len() - 1]).or_insert(0);
                *var += 1;
                break;
            }
        }

        for (key, val) in winners_dic {
            if !losers_dic.contains_key(&key) {
                ans[0].push(key);
            }
        }
        for (key, val) in losers_dic {
            if val == 1 {
                ans[1].push(key);
            }
        }
        ans[0].sort();
        ans[1].sort();
        ans
    }
}
// @lc code=end

fn main() {
    let mut test_arr: Vec<Vec<i32>> = Vec::new();
    test_arr.push(vec![1, 3]);
    test_arr.push(vec![2, 3]);
    test_arr.push(vec![3, 6]);
    test_arr.push(vec![5, 6]);
    test_arr.push(vec![5, 7]);
    test_arr.push(vec![4, 5]);
    test_arr.push(vec![4, 8]);
    test_arr.push(vec![4, 9]);
    test_arr.push(vec![10, 4]);
    test_arr.push(vec![10, 9]);
    let test_ans = Solution::find_winners(test_arr);

    for row in &test_ans {
        for val in row {
            print!("{}, ", val);
        }
        println!("onto next row");
    }
}

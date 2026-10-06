// Completed 10/5/2026
// Using monotonic decreasing
// O(n) time complexity
struct Solution;
/*
 * @lc app=leetcode id=239 lang=rust
 *
 * [239] Sliding Window Maximum
 */

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut ans: Vec<i32> = vec![];
        for (i, num) in nums.iter().enumerate() {
            if i as i32 >= k {
                ans.push(nums[queue[0] as usize]);
            }

            while queue.back().is_some() && nums[*queue.back().unwrap() as usize] < *num {
                queue.pop_back();
            }
            queue.push_back(i as i32);

            if !queue.is_empty() && queue[0] + k == i as i32 {
                queue.pop_front();
            }
        }
        ans.push(nums[queue[0] as usize]);
        ans
    }
}
// @lc code=end
fn main() {
    let test = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let test2 = vec![1];
    let ans = Solution::max_sliding_window(test, 3);
    for num in ans {
        println!("{}", num);
    }
}

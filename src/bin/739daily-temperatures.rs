// completed 10/4/2026
// Monotonic stack

struct Solution;
/*
 * @lc app=leetcode id=739 lang=rust
 *
 * [739] Daily Temperatures
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // keep a decreasing monotonic stack
        // keep track of the curr num on the stack that will be used for output
        let mut stack: Vec<i32> = vec![];
        let mut ans: Vec<i32> = vec![0; temperatures.len()];
        for (i, num) in temperatures.iter().enumerate() {
            while stack.last().is_some() && temperatures[*stack.last().unwrap() as usize] < *num {
                let j = *stack.last().unwrap();
                ans[j as usize] = i as i32 - j;
                stack.pop();
            }
            stack.push(i as i32);
        }
        ans
    }
}
// @lc code=end
fn main() {
    let test = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let ans = Solution::daily_temperatures(test);
    for num in ans {
        println!("{}", num);
    }
}

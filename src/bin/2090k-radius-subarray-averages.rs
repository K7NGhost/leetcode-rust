// Completed 4/12/2026
/*
 * @lc app=leetcode id=2090 lang=rust
 *
 * [2090] K Radius Subarray Averages
 */

use std::vec;

struct Solution;
// @lc code=start
impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // We know the length of the avg array is k * 2
        // looks like sliding window and the avg starts at the index of k until the end of nums array
        // **************************** Prefix solution
        // let window_size = k * 2 + 1;
        // let n = nums.len();
        // let mut averages = vec![-1; n];

        // if k == 0 {
        //     return nums;
        // }

        // if window_size > n as i32 {
        //     return averages;
        // }

        // let mut prefix: Vec<i64> = vec![0; n + 1];
        // for i in 0..n {
        //     prefix[i + 1] = prefix[i] + nums[i] as i64;
        // }

        // for i in k..(n as i32 - k) {
        //     let left_bound: i32 = i - k;
        //     let right_bound: i32 = i + k;
        //     let mut average: i64 = prefix[right_bound as usize + 1] - prefix[left_bound as usize];
        //     average = average / window_size as i64;
        //     averages[i as usize] = average as i32;
        // }
        // averages

        // **************************** Sliding window solution
        let n = nums.len();
        let k = k as usize;
        let range = (2 * k + 1) as i64;
        let mut ans = vec![-1; n];

        if range as usize > n {
            return ans;
        }

        let mut sum: i64 = nums[..range as usize]
            .iter()
            .copied()
            .map(|x| x as i64)
            .sum();

        for i in k..n - k {
            ans[i] = (sum / range) as i32;
            sum -= nums[i - k] as i64;

            if i + k + 1 < n {
                sum += nums[i + k + 1] as i64;
            }
        }

        ans
    }
}
// @lc code=end

fn main() {
    let test_arr = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
    let k = 3;
    let ans = Solution::get_averages(test_arr, k);

    for num in ans {
        println!("{}", num);
    }
}

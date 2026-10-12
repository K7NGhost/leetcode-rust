/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */

struct Solution;
// @lc code=start
use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // possible strategy
        // have two monotonic stacks
        // first keeps track of indices of nums1 increasing monotonic
        // second keeps track of indices of nums2 decreasing monotonic
        // we have to pop front until we hit a match with num1 and num2
        // if next num is greater place that in ans add to stack
        let mut indices: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums1.iter().enumerate() {
            indices.insert(*num, i);
        }
        let mut queue2 = VecDeque::new();
        let mut ans = vec![-1; nums1.len()];
        for (i, num) in nums2.iter().enumerate() {
            while queue2.back().is_some_and(|x| *x < *num) {
                if queue2.back().is_some_and(|x| indices.contains_key(x)) {
                    ans[*indices.get(queue2.back().unwrap()).unwrap()] = *num;
                }
                queue2.pop_back();
            }
            queue2.push_back(*num);
        }

        ans
    }
}
// @lc code=end
fn main() {
    let test1_1 = vec![4, 1, 2];
    let test1_2 = vec![1, 3, 4, 2];
    let test2_1 = vec![2, 4];
    let test2_2 = vec![1, 2, 3, 4];
    let ans = Solution::next_greater_element(test2_1, test2_2);
    for num in ans {
        println!("{}", num);
    }
}

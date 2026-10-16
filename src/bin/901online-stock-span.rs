// Completed 10/15/2025
// Not optimal solution copying and entire list
/*
 * @lc app=leetcode id=901 lang=rust
 *
 * [901] Online Stock Span
 */

// @lc code=start
struct StockSpanner {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stack: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        // game plan use monotonic stack for every pop inc ans
        // the stack should not change we take a copy of the stack
        let mut ans = 1;
        let mut stack_copy: Vec<i32> = self.stack.iter().copied().collect();

        while stack_copy.last().is_some_and(|x| *x <= price) {
            ans += 1;
            stack_copy.pop();
        }
        self.stack.push(price);

        ans
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
// @lc code=end
fn main() {
    let mut obj = StockSpanner::new();
    println!("{}", obj.next(100));
    println!("{}", obj.next(80));
    println!("{}", obj.next(60));
    println!("{}", obj.next(70));
    println!("{}", obj.next(60));
    println!("{}", obj.next(75));
    println!("{}", obj.next(85));
}

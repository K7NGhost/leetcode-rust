// Completed 9/27/2026
// Used Stack data structure
// not optimal
struct Solution;
/*
 * @lc app=leetcode id=71 lang=rust
 *
 * [71] Simplify Path
 */

// @lc code=start
impl Solution {
    pub fn simplify_path(path: String) -> String {
        // build a stack
        let mut stack: Vec<&str> = vec![];
        let mut ans_string = String::default();
        let mut clean_strings: Vec<&str> = path.split("/").collect();
        clean_strings.retain(|x| *x != "");
        for str in clean_strings {
            if str == ".." && !stack.is_empty() {
                stack.pop();
                continue;
            } else if str == ".." && stack.is_empty() {
                continue;
            } else if str == "." {
                continue;
            }
            stack.push(str);
        }
        stack.insert(0, "/");
        stack.iter().for_each(|x| print!("{}", x));
        if stack.len() == 1 {
            return stack[0].to_string();
        }
        ans_string = stack.join("/");
        ans_string.remove(0);
        println!("");
        //stack.iter().for_each(|x| println!("{}", x));
        println!("{}", ans_string);
        // based off the rules of the stack pop or push
        // the ending stack should give the the output
        ans_string
    }
}
// @lc code=end
fn main() {
    let test_string = String::from("/home//foo/");
    let test_string2 = String::from("/../");
    Solution::simplify_path(test_string2);
}

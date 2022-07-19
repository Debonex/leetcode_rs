/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

use super::Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x.cmp(&0) {
            Ordering::Less => false,
            Ordering::Equal => true,
            Ordering::Greater => {
                let mut num = x;
                let mut acc = 0;
                while num != 0 {
                    acc = acc * 10 + num % 10;
                    num /= 10;
                }
                acc == x
            }
        }
    }
}
// @lc code=end

#[test]
fn test_0009() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121));
    assert!(!Solution::is_palindrome(10));
}

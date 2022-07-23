/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */
use super::Solution;
// @lc code=start

fn is_match(left: &char, right: &char) -> bool {
    left == &'(' && right == &')' || left == &'[' && right == &']' || left == &'{' && right == &'}'
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                _ => {
                    if let Some(last_c) = stack.last() {
                        if is_match(last_c, &c) {
                            stack.pop();
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
// @lc code=end
#[test]
fn test_0020() {
    assert!(Solution::is_valid("()".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(!Solution::is_valid("([)]".to_string()));
    assert!(Solution::is_valid("{[]}".to_string()));
}

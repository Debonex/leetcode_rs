/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成&
 */
use super::Solution;
// @lc code=start
fn dfs(result: &mut Vec<String>, l_count: i32, r_count: i32, temp: &str) {
    if l_count == 0 && r_count == 0 && !temp.is_empty() {
        result.push(temp.to_string());
    } else {
        if l_count > 0 {
            dfs(result, l_count - 1, r_count, &format!("{}(", temp));
        }
        if r_count > l_count {
            dfs(result, l_count, r_count - 1, &format!("{})", temp));
        }
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        dfs(&mut result, n, n, "");

        result
    }
}
// @lc code=end
#[test]
fn test_0022() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string()
        ]
    );

    assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_string()]);
}

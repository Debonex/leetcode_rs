/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common_prefix = strs[0].clone();
        for str in strs.iter().skip(1) {
            common_prefix = Solution::common_prefix(&common_prefix, str);
            if common_prefix.is_empty() {
                break;
            }
        }
        common_prefix
    }

    fn common_prefix(str1: &str, str2: &str) -> String {
        let mut res = String::new();
        for (c1, c2) in str1.chars().zip(str2.chars()) {
            if c1 == c2 {
                res.push(c1);
            } else {
                break;
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn test_0014() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    assert_eq!(Solution::longest_common_prefix(strs), "fl".to_string());

    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    assert_eq!(Solution::longest_common_prefix(strs), "".to_string());
}

/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let len = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut result: Vec<char> = vec![];
        let mut idx;
        let mut flag;

        for i in 0..num_rows {
            if i == 0 || i == num_rows - 1 {
                idx = i as usize;
                while idx < len {
                    result.push(chars[idx]);
                    idx += 2 * (num_rows - 1) as usize;
                }
            } else {
                flag = true;
                idx = i as usize;
                while idx < len {
                    result.push(chars[idx]);
                    idx += match flag {
                        true => 2 * (num_rows - 1 - i) as usize,
                        false => 2 * (i as usize),
                    };
                    flag = !flag;
                }
            }
        }

        result.into_iter().collect()
    }
}
// @lc code=end

#[test]
fn test_0006() {
    let s = "PAYPALISHIRING".to_string();
    assert_eq!(Solution::convert(s, 3), "PAHNAPLSIIGYIR".to_string());

    let s = "PAYPALISHIRING".to_string();
    assert_eq!(Solution::convert(s, 4), "PINALSIGYAHRPI".to_string());

    let s = "A".to_string();
    assert_eq!(Solution::convert(s, 1), "A".to_string());
}

/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

use super::Solution;
// @lc code=start

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut symbol_list = vec![
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];
        let mut result = "".to_string();
        let mut num = num;
        while num > 0 {
            if symbol_list[0].1 > num {
                symbol_list.remove(0);
            } else {
                result.push_str(symbol_list[0].0);
                num -= symbol_list[0].1;
            }
        }
        result
    }
}
// @lc code=end

#[test]
fn test_0012() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
    assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
}

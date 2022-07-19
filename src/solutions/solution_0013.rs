/*
 * @lc app=leetcode.cn id=13 lang=rust
 *
 * [13] 罗马数字转整数
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let chars: Vec<char> = s.chars().rev().collect();
        for (i, char) in chars.iter().enumerate() {
            let value = Solution::value(char);
            res += if i > 0 && value < Solution::value(&chars[i - 1]) {
                -value
            } else {
                value
            }
        }

        res
    }

    fn value(char: &char) -> i32 {
        match char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        }
    }
}
// @lc code=end

#[test]
fn test_0013() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}

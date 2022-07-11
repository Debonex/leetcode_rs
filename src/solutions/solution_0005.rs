/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let chars: Vec<char> = s.chars().collect();
        let (mut max_l, mut max_r, mut max_len) = (0, 0, 1);
        let mut flags = vec![vec![false; len]; len];

        for i in 0..len {
            flags[i][i] = true;
        }

        for r_base in 1..len {
            for bias in 0..len - r_base {
                let l = bias;
                let r = bias + r_base;
                if chars[l] == chars[r] && (r - l == 1 || flags[l + 1][r - 1]) {
                    flags[l][r] = true;
                    if r - l + 1 > max_len {
                        max_l = l;
                        max_r = r;
                        max_len = r - l + 1;
                    }
                }
            }
        }

        s[max_l..=max_r].to_string()
    }
}
// @lc code=end

#[test]
fn test_5() {
    let s = "babad".to_string();
    assert_eq!(Solution::longest_palindrome(s), "bab".to_string());

    let s = "cbbd".to_string();
    assert_eq!(Solution::longest_palindrome(s), "bb".to_string());
}

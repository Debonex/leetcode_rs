/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

use super::Solution;
// @lc code=start
use std::{cmp::max, collections::HashMap};
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        let mut max_len = 0;
        let (mut l, mut r) = (0, 0);
        let mut map: HashMap<char, bool> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        // 双指针从左往右找
        while r < len && l < len {
            let r_char = chars.get(r).unwrap();
            match map.get(r_char) {
                Some(true) => {
                    // 如果有重复字符，l指针向右直到没有重复为止
                    loop {
                        let l_char = chars.get(l).unwrap();
                        *map.get_mut(l_char).unwrap() = false;
                        l += 1;
                        if l_char == r_char {
                            break;
                        }
                    }
                    *map.get_mut(r_char).unwrap() = true;
                }
                _ => {
                    map.insert(*r_char, true);
                    max_len = max(max_len, r - l + 1);
                }
            }
            r += 1;
        }

        max_len as i32
    }
}
// @lc code=end

#[test]
fn test_3() {
    assert_eq!(
        Solution::length_of_longest_substring("abcabcbb".to_string()),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring("bbbbb".to_string()),
        1
    );
    assert_eq!(
        Solution::length_of_longest_substring("pwwkew".to_string()),
        3
    );
}

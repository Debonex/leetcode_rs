/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

use super::Solution;
// @lc code=start
const MIN: &'static [usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 8];
const MAX: &'static [usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 7];
fn overflow(nums: &[usize], positive: bool) -> bool {
    let nums_len = nums.len();
    if nums_len < 10 {
        return false;
    } else if nums_len > 10 {
        return true;
    } else {
        let nums_compare = if positive { MAX } else { MIN };
        for i in 0..10 {
            if nums[i] < nums_compare[i] {
                return false;
            }
            if nums[i] > nums_compare[i] {
                return true;
            }
        }
        return false;
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s = s.trim();
        let mut nums: Vec<usize> = vec![];

        let negative = if s.starts_with('-') {
            s = &s[1..];
            true
        } else {
            if s.starts_with('+') {
                s = &s[1..];
            }
            false
        };

        for c in s.chars() {
            if c.is_digit(10) {
                nums.push(c as usize - '0' as usize);
            } else {
                break;
            }
        }

        let mut zero_idx = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] != 0 {
                zero_idx = i;
                break;
            }
            i += 1;
        }
        nums = if i == nums.len() {
            vec![0]
        } else {
            nums[zero_idx..].to_vec()
        };

        if overflow(&nums, !negative) {
            return if negative { i32::MIN } else { i32::MAX };
        }

        nums.iter().fold(0, |acc, x| {
            acc * 10 + if negative { -(*x as i32) } else { *x as i32 }
        })
    }
}
// @lc code=end

#[test]
fn test_8() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
}

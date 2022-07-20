/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

use super::Solution;
// @lc code=start
use std::cmp::Ordering;
const MIN: &[usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 8];
const MAX: &[usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 7];

impl Solution {
    fn overflow(nums: &[usize], positive: bool) -> bool {
        let nums_len = nums.len();

        match nums_len.cmp(&10) {
            Ordering::Less => false,
            Ordering::Equal => {
                let nums_compare = if positive { MAX } else { MIN };
                for i in 0..10 {
                    if nums[i] < nums_compare[i] {
                        return false;
                    }
                    if nums[i] > nums_compare[i] {
                        return true;
                    }
                }
                false
            }
            Ordering::Greater => true,
        }
    }

    pub fn reverse(x: i32) -> i32 {
        let mut nums: Vec<usize> = vec![];
        let positive = x > 0;
        let mut x = x;

        while x != 0 {
            nums.push((x % 10).unsigned_abs() as usize);
            x /= 10;
        }

        if Solution::overflow(&nums, positive) {
            return 0;
        }

        return nums.iter().fold(0, |acc, x| {
            acc * 10 + {
                if positive {
                    *x as i32
                } else {
                    -(*x as i32)
                }
            }
        });
    }
}
// @lc code=end

#[test]
fn test_0007() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);

    assert_eq!(Solution::reverse(-2147483412), -2143847412)
}

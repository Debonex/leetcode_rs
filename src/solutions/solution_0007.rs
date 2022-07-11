/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

use super::Solution;
// @lc code=start

const MIN: &'static [usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 8];
const MAX: &'static [usize] = &[2, 1, 4, 7, 4, 8, 3, 6, 4, 7];

impl Solution {
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

    pub fn reverse(x: i32) -> i32 {
        let mut nums: Vec<usize> = vec![];
        let positive = x > 0;
        let mut x = x;

        while x != 0 {
            nums.push((x % 10).abs() as usize);
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
fn test_7() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);

    assert_eq!(Solution::reverse(-2147483412), -2143847412)
}

/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

use super::Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let len = nums.len();
        let mut res = 0;
        let mut min_abs = i32::MAX;
        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, len - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                match (sum).cmp(&target) {
                    Ordering::Equal => {
                        return target;
                    }
                    Ordering::Greater => {
                        if sum - target < min_abs {
                            min_abs = sum - target;
                            res = sum;
                        }
                        r -= 1;
                        while r > l && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                    Ordering::Less => {
                        if target - sum < min_abs {
                            min_abs = target - sum;
                            res = sum;
                        }
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        res
    }
}
// @lc code=end

#[test]
fn test_0016() {
    let nums = vec![-1, 2, 1, -4];
    assert_eq!(Solution::three_sum_closest(nums, 1), 2);

    let nums = vec![0, 0, 0];
    assert_eq!(Solution::three_sum_closest(nums, 1), 0);
}

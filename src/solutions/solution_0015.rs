/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

use super::Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();

        for i in 0..len - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut l, mut r) = (i + 1, len - 1);
            while l < r {
                match (nums[i] + nums[r] + nums[l]).cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                    Ordering::Greater => {
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1;
                        }
                    }
                    Ordering::Less => {
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1;
                        }
                    }
                }
            }
        }
        result
    }
}
// @lc code=end

#[test]
fn test_0015() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    assert_eq!(
        Solution::three_sum(nums),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );

    let nums = vec![];
    assert_eq!(Solution::three_sum(nums), vec![] as Vec<Vec<i32>>);

    let nums = vec![0];
    assert_eq!(Solution::three_sum(nums), vec![] as Vec<Vec<i32>>);
}

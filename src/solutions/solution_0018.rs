/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */
use super::Solution;
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        if len < 4 {
            return result;
        }
        nums.sort_unstable();

        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }
                let (mut l, mut r) = (j + 1, len - 1);
                while l < r {
                    let sum = nums[i] as i128 + nums[j] as i128 + nums[l] as i128 + nums[r] as i128;
                    match (sum).cmp(&(target as i128)) {
                        Ordering::Equal => {
                            result.push(vec![nums[i], nums[j], nums[l], nums[r]]);
                            l += 1;
                            r -= 1;
                            while l < r && nums[l - 1] == nums[l] {
                                l += 1;
                            }
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
        }

        result
    }
}
// @lc code=end
#[test]
fn test_0018() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let res = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert_eq!(Solution::four_sum(nums, 0), res);

    let nums = vec![2, 2, 2, 2, 2];
    let res = vec![vec![2, 2, 2, 2]];
    assert_eq!(Solution::four_sum(nums, 8), res);

    let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
    assert_eq!(
        Solution::four_sum(nums, -294967296),
        vec![] as Vec<Vec<i32>>
    );
}

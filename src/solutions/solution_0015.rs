/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let result: Vec<Vec<i32>> = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();
        todo!()
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

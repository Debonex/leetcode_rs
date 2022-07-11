/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(complement_idx) = map.get(&complement) {
                return vec![*complement_idx, idx as i32];
            }
            map.insert(*num, idx as i32);
        }
        return vec![];
    }
}
// @lc code=end

#[test]
fn test_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);

    let nums = vec![3, 2, 4];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![1, 2]);

    let nums = vec![3, 3];
    let target = 6;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}

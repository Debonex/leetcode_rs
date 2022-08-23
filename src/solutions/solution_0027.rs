/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut p1, mut p2, len) = (0, 0, nums.len());
        while p2 < len {
            if nums[p2] != val {
                nums[p1] = nums[p2];
                p1 += 1;
            }
            p2 += 1;
        }
        p1 as i32
    }
}
// @lc code=end

#[test]
fn test_0027() {
    let mut nums = vec![3, 2, 2, 3];
    assert_eq!(Solution::remove_element(&mut nums, 3), 2);
    assert_eq!(&nums[0..2], &[2, 2]);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(Solution::remove_element(&mut nums, 2), 5);
    assert_eq!(&nums[0..5], &[0, 1, 3, 0, 4]);
}

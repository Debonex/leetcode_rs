/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (mut p1, mut p2) = (0, 1);
        let len = nums.len();
        while p2 < len {
            if nums[p2] != nums[p2 - 1] {
                p1 += 1;
                nums[p1] = nums[p2];
            }
            p2 += 1;
        }
        (p1 + 1) as i32
    }
}
// @lc code=end

#[test]
fn test_0026() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    assert_eq!(&nums[0..2], &[1, 2]);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
    assert_eq!(&nums[0..5], &[0, 1, 2, 3, 4])
}

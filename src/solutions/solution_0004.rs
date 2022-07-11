/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */
use super::Solution;
// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut list: Vec<i32> = Vec::new();
        let (mut i, mut j, len1, len2) = (0, 0, nums1.len(), nums2.len());
        while i < len1 || j < len2 {
            match (nums1.get(i), nums2.get(j)) {
                (Some(n1), Some(n2)) => {
                    if n1 <= n2 {
                        list.push(*n1);
                        i += 1;
                    } else {
                        list.push(*n2);
                        j += 1;
                    }
                }
                (Some(n1), None) => {
                    list.push(*n1);
                    i += 1;
                }
                (None, Some(n2)) => {
                    list.push(*n2);
                    j += 1;
                }
                _ => {}
            }
        }

        let len = list.len();
        if len % 2 == 0 {
            (list[len / 2] + list[len / 2 - 1]) as f64 / 2.0
        } else {
            list[len / 2] as f64
        }
    }
}
// @lc code=end

#[test]
fn test_4() {
    let (nums1, nums2) = (vec![1, 3], vec![2]);
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);

    let (nums1, nums2) = (vec![1, 2], vec![3, 4]);
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
}

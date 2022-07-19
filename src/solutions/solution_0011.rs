/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

use super::Solution;
// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut max_area = 0;
        while l < r {
            let area = (r - l) as i32 * (height[l].min(height[r]));
            max_area = area.max(max_area);
            if height[l] < height[r] {
                let lh = height[l];
                while l < r {
                    l += 1;
                    if height[l] > lh {
                        break;
                    }
                }
            } else {
                let rh = height[r];
                while r > l {
                    r -= 1;
                    if height[r] > rh {
                        break;
                    }
                }
            }
        }
        max_area
    }
}
// @lc code=end

#[test]
fn test_0011() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height), 49);

    let height = vec![1, 1];
    assert_eq!(Solution::max_area(height), 1);
}

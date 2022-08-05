/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
 */
use super::Solution;
use crate::data_structure::linked_list::ListNode;
// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.map(|mut n| match n.next {
            Some(mut m) => {
                n.next = Self::swap_pairs(m.next);
                m.next = Some(n);
                m
            }
            None => n,
        })
    }
}
// @lc code=end

#[test]
fn test_0024() {
    use crate::linked_list;

    assert_eq!(
        Solution::swap_pairs(linked_list!(1, 2, 3, 4)),
        linked_list!(2, 1, 4, 3)
    );

    assert_eq!(Solution::swap_pairs(None), None);

    assert_eq!(Solution::swap_pairs(linked_list!(1)), linked_list!(1));
}

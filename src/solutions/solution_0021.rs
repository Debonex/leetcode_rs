/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
 */

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
use super::Solution;
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut list1, mut list2) = (list1, list2);
        let mut dummy = ListNode::new(-1);
        let mut tail = &mut dummy;

        while let (Some(n1), Some(n2)) = (list1.as_ref(), list2.as_ref()) {
            if n1.val < n2.val {
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        tail.next = if list1.is_none() { list2 } else { list1 };
        dummy.next
    }
}
// @lc code=end

#[test]
fn test_0021() {
    use crate::linked_list;
    let l1 = linked_list!(1, 2, 4);
    let l2 = linked_list!(1, 3, 4);
    let res = linked_list!(1, 1, 2, 3, 4, 4);
    assert_eq!(Solution::merge_two_lists(l1, l2), res);

    assert_eq!(
        Solution::merge_two_lists(linked_list!(), linked_list!()),
        linked_list!()
    );

    assert_eq!(
        Solution::merge_two_lists(linked_list!(), linked_list!(0)),
        linked_list!(0)
    );
}

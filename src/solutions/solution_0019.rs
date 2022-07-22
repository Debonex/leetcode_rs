/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut len = 0;
        let mut current = &mut dummy.as_mut().unwrap().next;
        while let Some(node) = current {
            len += 1;
            current = &mut node.next;
        }

        current = &mut dummy;
        for _ in 0..len - n {
            current = &mut current.as_mut().unwrap().next;
        }

        let remove = &mut current.as_mut().unwrap().next;
        current.as_mut().unwrap().next = remove.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}
// @lc code=end

#[test]
fn test_0019() {
    use crate::linked_list;

    let head = linked_list!(1, 2, 3, 4, 5);
    let res = linked_list!(1, 2, 3, 5);
    assert_eq!(Solution::remove_nth_from_end(head, 2), res);

    let head = linked_list!(1);
    let res = linked_list!();
    assert_eq!(Solution::remove_nth_from_end(head, 1), res);

    let head = linked_list!(1, 2);
    let res = linked_list!(1);
    assert_eq!(Solution::remove_nth_from_end(head, 1), res);
}

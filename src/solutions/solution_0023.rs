/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并K个升序链表
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
fn merge_node_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut list1, mut list2) = (list1, list2);
    let mut dummy = ListNode::new(-1);
    let mut tail = &mut dummy;

    while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
        if node1.val < node2.val {
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

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut res = None;
        for list in lists {
            res = merge_node_lists(res, list);
        }

        res
    }
}
// @lc code=end
#[test]
fn solution_0023() {
    use crate::linked_list;
    let lists = vec![
        linked_list!(1, 4, 5),
        linked_list!(1, 3, 4),
        linked_list!(2, 6),
    ];
    assert_eq!(
        Solution::merge_k_lists(lists),
        linked_list!(1, 1, 2, 3, 4, 4, 5, 6)
    );

    assert_eq!(
        Solution::merge_k_lists(vec![linked_list!()]),
        linked_list!()
    );

    assert_eq!(Solution::merge_k_lists(vec![]), linked_list!());
}

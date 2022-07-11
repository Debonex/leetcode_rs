/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut n1, mut n2) = (l1, l2);
        let mut carry = 0;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut current = &mut dummy;
        loop {
            if n1 == None && n2 == None {
                if carry > 0 {
                    current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
                }
                break;
            }

            let n1_val = match n1 {
                Some(node) => {
                    n1 = node.next;
                    node.val
                }
                None => 0,
            };

            let n2_val = match n2 {
                Some(node) => {
                    n2 = node.next;
                    node.val
                }
                None => 0,
            };

            let mut sum = n1_val + n2_val + carry;
            if sum > 9 {
                sum -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            current = &mut current.as_mut().unwrap().next;
        }

        dummy.unwrap().next
    }
}
// @lc code=end

#[test]
fn test_2() {
    use crate::linked_list;

    let l1 = linked_list!(2, 4, 3);
    let l2 = linked_list!(5, 6, 4);
    let res = linked_list!(7, 0, 8);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);

    let l1 = linked_list!(0);
    let l2 = linked_list!(0);
    let res = linked_list!(0);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);

    let l1 = linked_list!(9, 9, 9, 9, 9, 9, 9);
    let l2 = linked_list!(9, 9, 9, 9);
    let res = linked_list!(8, 9, 9, 9, 0, 0, 0, 1);
    assert_eq!(Solution::add_two_numbers(l1, l2), res);
}

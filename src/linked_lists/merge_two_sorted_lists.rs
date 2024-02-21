//! https://leetcode.com/problems/merge-two-sorted-lists/

use std::mem;

use crate::data_structures::linked_list::ListNode;

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;

    while list1.is_some() && list2.is_some() {
        let l1 = &mut list1;
        let l2 = &mut list2;

        if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
            mem::swap(current, l1);
            mem::swap(l1, &mut current.as_mut().unwrap().next);
        } else {
            mem::swap(current, l2);
            mem::swap(l2, &mut current.as_mut().unwrap().next);
        }
        current = &mut current.as_mut().unwrap().next;
    }

    if list1.is_some() {
        *current = list1;
    } else {
        *current = list2;
    }

    head
}

#[cfg(test)]
mod test {
    use crate::linked_lists::merge_two_sorted_lists::*;

    #[test]
    fn merge_two_nonempty() {
        assert_eq!(
            ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]),
            merge_two_lists(ListNode::from_vec(vec![1, 2, 4]), ListNode::from_vec(vec![1, 3, 4])),
        );
    }

    #[test]
    fn merge_two_empty() {
        assert_eq!(
            ListNode::from_vec(vec![]),
            merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![])),
        );
    }

    #[test]
    fn merge_empty_with_nonempty() {
        assert_eq!(
            ListNode::from_vec(vec![0]),
            merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![0])),
        );
    }
}
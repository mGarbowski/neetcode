//! https://leetcode.com/problems/reverse-linked-list/

use std::mem;
use crate::data_structures::linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;
    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

pub fn reverse_list_recursive(current: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match current {
        None => prev,
        Some(mut node) => reverse_list_recursive(mem::replace(&mut node.next, prev), Some(node))
    }
}

#[cfg(test)]
mod test {
    use crate::data_structures::linked_list::ListNode;
    use crate::linked_lists::reverse_linked_list::*;

    #[test]
    fn reverse_nonempty() {
        assert_eq!(
            ListNode::from_vec(vec![5, 4, 3, 2, 1]),
            reverse_list(ListNode::from_vec(vec![1, 2, 3, 4, 5]))
        );
    }

    #[test]
    fn reverse_length_2() {
        assert_eq!(
            ListNode::from_vec(vec![1, 2]),
            reverse_list(ListNode::from_vec(vec![2, 1]))
        );
    }

    #[test]
    fn reverse_empty() {
        assert_eq!(
            ListNode::from_vec(vec![]),
            reverse_list(ListNode::from_vec(vec![]))
        );
    }

    #[test]
    fn reverse_nonempty_recursive() {
        assert_eq!(
            ListNode::from_vec(vec![5, 4, 3, 2, 1]),
            reverse_list_recursive(ListNode::from_vec(vec![1, 2, 3, 4, 5]), None)
        );
    }
}
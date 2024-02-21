#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        for &val in vec.iter() {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }

        head
    }

    pub fn to_vec(self: Box<ListNode>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = Some(self);

        while let Some(inner) = current {
            result.push(inner.val);
            current = inner.next;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::data_structures::linked_list::*;

    #[test]
    fn as_vec_nonempty() {
        let mut head = Box::new(ListNode::new(1));
        head.next = Some(Box::new(ListNode::new(2)));
        head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        assert_eq!(vec![1, 2, 3], head.to_vec())
    }

    #[test]
    fn from_vec_nonempty() {
        let mut node = &mut ListNode::from_vec(vec![1, 2, 3]);
        assert_eq!(1, node.as_ref().unwrap().val);
        node = &mut node.as_mut().unwrap().next;
        assert_eq!(2, node.as_ref().unwrap().val);
        node = &mut node.as_mut().unwrap().next;
        assert_eq!(3, node.as_ref().unwrap().val);
        node = &mut node.as_mut().unwrap().next;
        assert!(node.is_none());
    }

    #[test]
    fn from_vec_empty() {
        assert_eq!(None, ListNode::from_vec(vec![]));
    }

    #[test]
    fn equal_empty_lists() {
        assert_eq!(ListNode::from_vec(vec![]), ListNode::from_vec(vec![]));
    }

    #[test]
    fn equal_length_1() {
        assert_eq!(ListNode::from_vec(vec![1]), ListNode::from_vec(vec![1]));
    }

    #[test]
    fn equal_length_2() {
        assert_eq!(ListNode::from_vec(vec![1, 2]), ListNode::from_vec(vec![1, 2]));
    }

    #[test]
    fn equal_length_5() {
        assert_eq!(ListNode::from_vec(vec![1, 2, 3, 4, 5]), ListNode::from_vec(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn not_equal_length_1() {
        assert_ne!(ListNode::from_vec(vec![1]), ListNode::from_vec(vec![2]));
    }

    #[test]
    fn not_equal_length_2() {
        assert_ne!(ListNode::from_vec(vec![1, 2]), ListNode::from_vec(vec![3, 4]));
    }

    #[test]
    fn not_equal_length_5() {
        assert_ne!(ListNode::from_vec(vec![1, 2, 3, 4, 5]), ListNode::from_vec(vec![1, 6, 4, 2, 1]));
    }

    #[test]
    fn not_equal_different_lengths() {
        assert_ne!(ListNode::from_vec(vec![1, 2, 3]), ListNode::from_vec(vec![1, 2, 3, 4, 5]));
    }
}
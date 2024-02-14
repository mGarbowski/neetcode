//! https://leetcode.com/problems/next-greater-element-i/

use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut next_greater: HashMap<i32, i32> = HashMap::new();
    for num in nums2 {
        while !stack.is_empty() && *stack.last().unwrap() < num {
            next_greater.insert(stack.pop().unwrap(), num);
        }
        stack.push(num);
    }

    nums1.iter()
        .map(|num| *next_greater.get(num).unwrap_or(&-1))
        .collect()
}



#[cfg(test)]
mod test {
    use crate::stack::next_greater_element_i::*;

    #[test]
    fn next_greater_1() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }
}
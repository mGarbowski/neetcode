//! https://leetcode.com/problems/contains-duplicate/description/
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        let duplicate = !set.insert(num);
        if duplicate {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::contains_duplicate::*;

    #[test]
    fn contains_duplicate_true() {
        let vec = vec![1, 2, 3, 1];
        assert_eq!(true, contains_duplicate(vec))
    }

    #[test]
    fn contains_duplicate_false() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(false, contains_duplicate(vec))
    }
}
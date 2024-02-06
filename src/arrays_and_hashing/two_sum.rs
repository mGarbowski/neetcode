//! https://leetcode.com/problems/two-sum/
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(other_idx) = diffs.get(&diff) {
            return vec![idx as i32, *other_idx as i32];
        }
        diffs.insert(num, idx);
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::two_sum::*;

    #[test]
    fn two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result)
    }

    #[test]
    fn two_sum_2() {
        let nums = vec![3, 3];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result)
    }

    #[test]
    fn two_sum_3() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![1, 2], result)
    }
}
//! https://leetcode.com/problems/longest-consecutive-sequence/

use std::cmp::max;
use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums_set: HashSet<i32> = nums.into_iter().collect();
    let mut max_length = 0;

    for sequence_start in &nums_set {
        if nums_set.contains(&(sequence_start - 1)) {
            continue;
        }

        let mut sequence_len = 1;
        let mut next_num = sequence_start + 1;
        while nums_set.contains(&next_num) {
            sequence_len += 1;
            next_num += 1;
        }

        max_length = max(max_length, sequence_len);
    }

    max_length
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::longest_consecutive_sequence::*;

    #[test]
    fn lcs_1() {
        assert_eq!(4, longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }

    #[test]
    fn lcs_2() {
        assert_eq!(9, longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]));
    }

    #[test]
    fn lcs_3() {
        assert_eq!(12, longest_consecutive(vec![-4, -1, 4, -5, 1, -6, 9, -6, 0, 2, 2, 7, 0, 9, -3, 8, 9, -2, -6, 5, 0, 3, 4, -2]));
    }
}


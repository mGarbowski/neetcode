//! https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

use std::cmp::Ordering;

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let sum = numbers[left] + numbers[right];
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => { return vec![(left + 1) as i32, (right + 1) as i32]; }
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use crate::two_pointers::two_sum_2::two_sum;

    #[test]
    fn two_sum_1() {
        assert_eq!(vec![1, 2], two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn two_sum_2() {
        assert_eq!(vec![1, 3], two_sum(vec![2, 3, 4], 6));
    }
}

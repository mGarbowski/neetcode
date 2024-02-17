//! https://leetcode.com/problems/3sum/description/

use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut triplets = HashSet::new();
    let mut nums = nums;
    nums.sort();

    for i in 0..n-2 {
        let mut left = i+1;
        let mut right = n-1;
        while left < right {
            if nums[i] + nums[left] + nums[right] < 0 {
                left += 1;
            } else if nums[i] + nums[left] + nums[right] > 0 {
                right -= 1;
            } else {
                triplets.insert(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;
            }
        }
    }

    triplets.into_iter().collect()
}

#[cfg(test)]
mod test {
    use crate::sorted;
    use crate::two_pointers::three_sum::*;

    #[test]
    fn three_sum_1() {
        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            sorted(three_sum(vec![-1, 0, 1, 2, -1, -4]))
        );
    }

    #[test]
    fn three_sum_2() {
        assert_eq!(
            vec![
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-3, -1, 4],
                vec![-3, 0, 3],
                vec![-3, 1, 2],
                vec![-2, -1, 3],
                vec![-2, 0, 2],
                vec![-1, -1, 2],
                vec![-1, 0, 1],
            ],
            sorted(three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]))
        );
    }
}
//! https://leetcode.com/problems/3sum/description/

use std::collections::HashSet;

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();
    let mut result: HashSet<(i32, i32, i32)> = HashSet::new();

    for left_idx in 0..=nums.len() - 3 {
        for right_idx in (left_idx + 2..nums.len()).rev() {
            let left = nums[left_idx];
            let right = nums[right_idx];
            if left * right > 0 {
                break;
            }

            let target = -(left + right);
            if target <= 0 {
                let mut mid_idx = left_idx + 1;
                while mid_idx < right_idx && nums[mid_idx] <= 0 {
                    if nums[mid_idx] == target {
                        result.insert((left, target, right));
                    }
                    mid_idx += 1;
                }
            } else {
                let mut mid_idx = right_idx - 1;
                while mid_idx > left_idx && nums[mid_idx] > 0 {
                    if nums[mid_idx] == target {
                        result.insert((left, target, right));
                    }
                    mid_idx -= 1;
                }
            }
        }
    }

    result.into_iter()
        .map(|(a, b, c)| vec![a, b, c])
        .collect()
}

#[cfg(test)]
mod test {
    use crate::two_pointers::three_sum::*;
    use crate::sorted;

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
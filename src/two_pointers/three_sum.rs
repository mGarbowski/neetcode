//! https://leetcode.com/problems/3sum/


pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut triplets = vec![];
    let mut nums = nums;
    nums.sort();

    for (i, &a) in nums.iter().enumerate() {
        if i > 0 && a == nums[i-1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = n - 1;
        while left < right {
            let sum = a + nums[left] + nums[right];
            if sum < 0 {
                left += 1;
            } else if sum > 0 {
                right -= 1;
            } else {
                let left_val = nums[left];
                let right_val = nums[right];
                triplets.push(vec![a, left_val, right_val]);

                right -= 1;
                while left < right && nums[right] == right_val {
                    right -= 1;
                }

                left += 1;
                while left < right && nums[left] == left_val {
                    left += 1;
                }
            }
        }
    }

    triplets
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
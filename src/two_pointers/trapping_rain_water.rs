//! https://leetcode.com/problems/trapping-rain-water/

use std::cmp::max;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut total_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_left_height = height[left];
    let mut max_right_height = height[right];

    while left < right {
        if max_left_height <= max_right_height {
            left += 1;
            max_left_height = max(height[left], max_left_height);
            total_area += max_left_height - height[left];
        } else {
            right -= 1;
            max_right_height = max(height[right], max_right_height);
            total_area += max_right_height - height[right];
        }
    }

    total_area
}

#[cfg(test)]
mod test {
    use crate::two_pointers::trapping_rain_water::*;

    #[test]
    fn trap_1() {
        assert_eq!(6, trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
    }

    #[test]
    fn trap_length_1() {
        assert_eq!(0, trap(vec![0]));
    }
}

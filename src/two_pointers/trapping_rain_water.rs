//! https://leetcode.com/problems/trapping-rain-water/

use std::cmp::{max, min};

pub fn trap(height: Vec<i32>) -> i32 {
    let mut total_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_left_height = height[left];
    let mut max_right_height = height[right];
    let mut current_level;

    while left < right {
        let advancing_left = max_left_height <= max_right_height;
        if advancing_left {
            left += 1;
            current_level = height[left];
        } else {
            right -= 1;
            current_level = height[right];
        }

        let bounding_height = min(max_left_height, max_right_height);
        let bar_height = max(bounding_height - current_level, 0);
        total_area += bar_height;

        if advancing_left {
            max_left_height = max(current_level, max_left_height);
        } else {
            max_right_height = max(current_level, max_right_height);
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

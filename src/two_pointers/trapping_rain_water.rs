//! https://leetcode.com/problems/trapping-rain-water/

use std::cmp::{max, min};

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n <= 2 {
        return 0;
    }

    let mut max_height_to_left = vec![0; n];
    for i in 1..n {
        max_height_to_left[i] = max(max_height_to_left[i - 1], height[i - 1]);
    }

    let mut max_height_to_right = vec![0; n];
    for i in (0..=n - 2).rev() {
        max_height_to_right[i] = max(max_height_to_right[i + 1], height[i + 1]);
    }

    let mut total_area = 0;
    for i in 0..n {
        let bounding_height = min(max_height_to_left[i], max_height_to_right[i]);
        let water_area = bounding_height - height[i];
        total_area += max(water_area, 0);
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

//! https://leetcode.com/problems/container-with-most-water/

use std::cmp::{max, min};

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let width = (right - left) as i32;
        let min_height = min(height[left], height[right]);
        max_area = max(width * min_height, max_area);

        if height[left] <= height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod test {
    use crate::two_pointers::container_with_most_water::*;

    #[test]
    fn max_area_1() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }
}
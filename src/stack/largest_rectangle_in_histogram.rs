//! https://leetcode.com/problems/largest-rectangle-in-histogram/

use std::cmp::max;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    for (idx, height) in heights.iter().enumerate() {
        let mut left_border = idx;
        while left_border > 0 && heights[left_border - 1] >= *height {
            left_border -= 1;
        }

        let mut right_border = idx;
        while right_border + 1 < heights.len() && heights[right_border + 1] >= *height {
            right_border += 1;
        }

        let width = (right_border - left_border + 1) as i32;
        max_area = max(width * height, max_area);
    }

    max_area
}

#[cfg(test)]
mod test {
    use crate::stack::largest_rectangle_in_histogram::largest_rectangle_area;

    #[test]
    fn largest_rectangle_1() {
        assert_eq!(10, largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
    }

    #[test]
    fn largest_rectangle_2() {
        assert_eq!(4, largest_rectangle_area(vec![2, 4]));
    }

    #[test]
    fn largest_rectangle_3() {
        assert_eq!(4, largest_rectangle_area(vec![2, 3]));
    }
}
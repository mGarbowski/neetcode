//! https://leetcode.com/problems/largest-rectangle-in-histogram/

use std::cmp::max;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<(usize, i32)> = vec![];

    for (idx, height) in heights.iter().enumerate() {
        let mut new_start = idx;
        while !stack.is_empty() && stack.last().unwrap().1 > *height {
            let (start, other_height) = stack.pop().unwrap();
            new_start = start;
            let width = (idx - start) as i32;
            max_area = max(max_area, width * other_height)
        }
        stack.push((new_start, *height));
    }

    for (start, height) in stack {
        let width = (heights.len() - start) as i32;
        max_area = max(max_area, width * height)
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
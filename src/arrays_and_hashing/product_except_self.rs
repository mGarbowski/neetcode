//! https://leetcode.com/problems/product-of-array-except-self/

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];

    let mut prefix_prod = 1;
    for i in 1..nums.len() {
        prefix_prod *= nums[i - 1];
        result[i] *= prefix_prod;
    }

    let mut suffix_prod = 1;
    for i in (0..nums.len() - 1).rev() {
        suffix_prod *= nums[i + 1];
        result[i] *= suffix_prod;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::product_except_self::*;

    #[test]
    fn product_except_self_positive() {
        assert_eq!(vec![24, 12, 8, 6], product_except_self(vec![1, 2, 3, 4]))
    }

    #[test]
    fn product_except_self_zero() {
        assert_eq!(vec![0, 0, 9, 0, 0], product_except_self(vec![-1, 1, 0, -3, 3]))
    }
}
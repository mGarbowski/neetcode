//! https://leetcode.com/problems/top-k-frequent-elements/

use crate::arrays_and_hashing::count_elements;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n_counts = nums.len() + 1;
    let counts = count_elements(nums);
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; n_counts];

    for (num, count) in counts {
        buckets.get_mut(count).unwrap().push(num)
    }

    let mut result = Vec::new();
    let mut idx = n_counts - 1;
    while result.len() < k as usize {
        let bucket = buckets.get_mut(idx).unwrap();
        if bucket.len() > 0 {
            result.append(bucket);
        }

        idx -= 1;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::top_k_frequent::*;

    #[test]
    fn top_k_frequent_elements() {
        assert_eq!(vec![1, 2], top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2))
    }
}
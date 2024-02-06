use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

/// https://leetcode.com/problems/contains-duplicate/description/
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        let duplicate = !set.insert(num);
        if duplicate {
            return true;
        }
    }

    false
}

fn count_chars(s: String) -> HashMap<char, i32> {
    let mut s_chars: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        if let Some(count) = s_chars.get_mut(&char) {
            *count += 1;
        } else {
            s_chars.insert(char, 1);
        }
    }
    s_chars
}

/// https://leetcode.com/problems/valid-anagram/
fn is_anagram(s: String, t: String) -> bool {
    let s_chars = count_chars(s);
    let t_chars = count_chars(t);
    s_chars == t_chars
}

/// https://leetcode.com/problems/two-sum/
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_idxs: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
    nums_idxs.sort_by_key(|ni| ni.1);

    let mut low = 0;
    let mut high = nums_idxs.len() - 1;
    let mut sum = nums_idxs[low].1 + nums_idxs[high].1;

    while sum != target {
        if sum > target {
            high -= 1;
        } else {
            low += 1;
        }
        sum = nums_idxs[low].1 + nums_idxs[high].1;
    }

    vec![
        nums_idxs[low].0 as i32,
        nums_idxs[high].0 as i32
    ]
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn contains_duplicate_true() {
        let vec = vec![1, 2, 3, 1];
        assert_eq!(true, contains_duplicate(vec))
    }

    #[test]
    fn contains_duplicate_false() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!(false, contains_duplicate(vec))
    }

    #[test]
    fn is_anagram_true() {
        let s1 = "anagram".to_owned();
        let s2 = "nagaram".to_owned();
        assert!(is_anagram(s1, s2))
    }

    #[test]
    fn is_anagram_false() {
        let s1 = "car".to_owned();
        let s2 = "rat".to_owned();
        assert!(!is_anagram(s1, s2))
    }

    #[test]
    fn two_sum_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(vec![0, 1], two_sum(nums, target))
    }

    #[test]
    fn two_sum_2() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(vec![0, 1], two_sum(nums, target))
    }

    #[test]
    fn two_sum_3() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(vec![1, 2], two_sum(nums, target))
    }
}

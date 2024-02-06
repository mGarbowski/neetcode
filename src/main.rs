use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn main() {
    println!("Hello, world!");
}

fn count_chars(s: String) -> HashMap<char, i32> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        if let Some(count) = char_counts.get_mut(&char) {
            *count += 1;
        } else {
            char_counts.insert(char, 1);
        }
    }
    char_counts
}

fn count_elements<T>(elements: Vec<T>) -> HashMap<T, i32>
    where T: Eq + Hash
{
    let mut counts = HashMap::new();
    for element in elements {
        *counts.entry(element).or_default() += 1
    }
    counts
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

/// https://leetcode.com/problems/valid-anagram/
fn is_anagram(s: String, t: String) -> bool {
    let s_chars = count_chars(s);
    let t_chars = count_chars(t);
    s_chars == t_chars
}

/// https://leetcode.com/problems/two-sum/
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut diffs = HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        let diff = target - num;
        if let Some(other_idx) = diffs.get(&diff) {
            return vec![idx as i32, *other_idx as i32];
        }
        diffs.insert(num, idx);
    }

    unreachable!();
}


/// https://leetcode.com/problems/group-anagrams/
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for str in strs {
        let mut chars: Vec<char> = str.clone().chars().collect();
        chars.sort();
        let sorted: String = chars.into_iter().collect();
        groups.entry(sorted).or_default().push(str);
    }

    groups.values().cloned().collect()
}

/// https://leetcode.com/problems/top-k-frequent-elements/
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let counts = count_elements(nums);
    let mut entries: Vec<(i32, i32)> = counts.into_iter().collect();
    entries.sort_by_key(|en| -en.1);
    entries.iter()
        .take(k as usize)
        .cloned()
        .map(|(num, _)| num)
        .collect()
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
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result)
    }

    #[test]
    fn two_sum_2() {
        let nums = vec![3, 3];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![0, 1], result)
    }

    #[test]
    fn two_sum_3() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut result = two_sum(nums, target);
        result.sort();
        assert_eq!(vec![1, 2], result)
    }

    fn sorted(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }
        groups.sort_by_key(|strs| strs.len());
        groups
    }

    #[test]
    fn group_anagrams_multiple_groups() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];

        let expected = vec![
            vec!["bat"],
            vec!["nat", "tan"],
            vec!["ate", "eat", "tea"],
        ];

        let result = sorted(group_anagrams(strs));
        assert_eq!(result, expected);
    }

    #[test]
    fn top_k_frequent_elements() {
        assert_eq!(vec![1, 2], top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2))
    }

    #[test]
    fn count_elements_multiple() {
        assert_eq!(HashMap::from([(1, 3), (2, 2), (3, 1)]), count_elements(vec![1, 1, 1, 2, 2, 3]))
    }
}

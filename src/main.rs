use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::zip;

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

fn maps_equal<K, V>(map_1: &HashMap<K, V>, map_2: &HashMap<K, V>) -> bool
where
    K: Eq + Hash,
    V: Eq
{
    if map_1.len() != map_2.len() {
        return false;
    }

    for (key, value_1) in map_1 {
        if let Some(value_2) = map_2.get(key){
            if value_1 != value_2 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

/// https://leetcode.com/problems/group-anagrams/
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: Vec<(Vec<String>, HashMap<char, i32>)> = Vec::new();
    let counts: Vec<HashMap<char, i32>> = strs.iter()
        .map(|str| count_chars(str.clone()))
        .collect();

    for (str, char_counts) in zip(strs, counts) {
        let mut is_new = true;
        for (strings, group_counts) in &mut groups {
            if maps_equal(&group_counts, &char_counts) {
                strings.push(str.clone());
                is_new = false;
                break;
            }
        }

        if is_new {
            groups.push((vec![str], char_counts.clone()));
        }
    }

    groups.iter()
        .map(|(strings, _)| strings.clone())
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
        for mut group in &mut groups {
            group.sort();
        }
        groups.sort_by_key(|strs| strs.len());
        groups
    }

    #[test]
    fn group_anagrams_multiple_groups() {
        let strs = vec!["eat".to_owned(),"tea".to_owned(),"tan".to_owned(),"ate".to_owned(),"nat".to_owned(),"bat".to_owned()];
        let expected = vec![vec!["bat"],vec!["nat","tan"],vec!["ate","eat","tea"]];
        let result = sorted(group_anagrams(strs));
        assert_eq!(result, expected);
    }
}

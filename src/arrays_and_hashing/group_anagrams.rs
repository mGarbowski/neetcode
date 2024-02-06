//! https://leetcode.com/problems/group-anagrams/
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for str in strs {
        let mut chars: Vec<char> = str.clone().chars().collect();
        chars.sort();
        let sorted: String = chars.into_iter().collect();
        groups.entry(sorted).or_default().push(str);
    }

    groups.values().cloned().collect()
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::group_anagrams::*;

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
}
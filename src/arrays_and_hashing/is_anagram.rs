//! https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let s_chars = count_chars(s);
    let t_chars = count_chars(t);
    s_chars == t_chars
}

fn count_chars(s: String) -> HashMap<char, i32> {
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        *char_counts.entry(char).or_default() += 1;
    }
    char_counts
}

#[cfg(test)]
mod test {
    use crate::arrays_and_hashing::is_anagram::*;

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
}
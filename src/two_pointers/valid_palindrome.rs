//! https://leetcode.com/problems/valid-palindrome/

pub fn is_palindrome(s: String) -> bool {
    let clean_string: Vec<char> = s.chars()
        .map(|char| char.to_ascii_lowercase())
        .filter(|char| ('a'..='z').contains(char) || ('0'..='9').contains(char))
        .collect();

    if clean_string.len() == 0 {
        return true
    }

    let mut left = 0;
    let mut right = clean_string.len() - 1;
    while left < right {
        if clean_string[left] != clean_string[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::two_pointers::valid_palindrome::*;

    #[test]
    fn is_palindrome_true() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()))
    }

    #[test]
    fn is_palindrome_empty() {
        assert!(is_palindrome(" ".to_string()))
    }

    #[test]
    fn is_palindrome_false_1() {
        assert!(!is_palindrome("race a car".to_string()))
    }

    #[test]
    fn is_palindrome_false_2() {
        assert!(is_palindrome("a.".to_string()))
    }

    #[test]
    fn is_palindrome_false_3() {
        assert!(!is_palindrome("0P".to_string()))
    }
}
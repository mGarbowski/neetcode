//! https://leetcode.com/problems/valid-parentheses/

use std::collections::LinkedList;

fn is_valid(s: String) -> bool {
    let mut stack = LinkedList::new();

    for char in s.chars() {
        if char == '(' || char == '[' || char == '{' {
            stack.push_back(char)
        } else {
            if stack.is_empty() {
                return false
            }

            let tail = *stack.back().unwrap();
            if (tail == '(' && char == ')') || (tail == '[' && char == ']') || (tail == '{' && char == '}') {
                stack.pop_back();
            }  else {
                return false
            }
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod test {
    use crate::stack::valid_parentheses::is_valid;

    #[test]
    fn is_valid_simple() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn is_valid_multiple() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn is_valid_nested() {
        assert!(is_valid("([])".to_string()));
    }

    #[test]
    fn is_not_valid() {
        assert!(!is_valid("(]".to_string()));
    }
}
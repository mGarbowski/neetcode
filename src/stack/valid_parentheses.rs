//! https://leetcode.com/problems/valid-parentheses/

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for char in s.chars() {
        match (stack.last(), char) {
            (_, '(') | (_, '{') | (_, '[') => stack.push(char),
            (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => { stack.pop(); }
            _ => return false,
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
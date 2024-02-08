//! https://leetcode.com/problems/generate-parentheses/

use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations = HashSet::new();
    combinations.insert("()".to_string());
    for _ in 1..n {
        combinations = combinations.iter()
            .map(|cmb| next_combinations(cmb))
            .fold(HashSet::new(), |acc, set| acc.union(&set).cloned().collect());
    }

    combinations.into_iter().collect()
}

fn next_combinations(combination: &str) -> HashSet<String> {
    let mut combinations = HashSet::new();
    for start_idx in 0..combination.len() {
        for end_idx in valid_other_ends(combination, start_idx) {
            combinations.insert(add_parentheses(&combination, start_idx, end_idx));
        }
    }

    combinations
}

fn valid_other_ends(combination: &str, start_idx: usize) -> Vec<usize> {
    let mut end_idxs = vec![start_idx];
    let mut stack = vec![];

    for (idx, char) in combination.chars().enumerate().skip(start_idx) {
        match char {
            '(' => {stack.push('(');},
            ')' => {
                if stack.pop().is_none() {
                    break;
                }

                if stack.is_empty() {
                    end_idxs.push(idx+1);
                }
            }
            _ => unreachable!()
        }
    }

    end_idxs
}

fn add_parentheses(combination: &str, left_idx: usize, right_idx: usize) -> String {
    let left = &combination[..left_idx];
    let mid = &combination[left_idx..right_idx];
    let right = &combination[right_idx..];
    format!("{left}({mid}){right}")
}

#[cfg(test)]
mod test {
    use crate::*;
    use crate::stack::generate_parentheses::*;


    #[test]
    fn add_parentheses_1() {
        assert_eq!("(())()".to_string(), add_parentheses("()()", 0, 2));
    }
    #[test]
    fn next_combination_1() {
        assert_eq!(
            next_combinations("()()"),
            HashSet::from([
                "()()()".to_string(),
                "(())()".to_string(),
                "(()())".to_string(),
                "()(())".to_string()
            ])
        )
    }

    #[test]
    fn generate_parenthesis_1() {
        assert_eq!(
            vec!["()"],
            generate_parenthesis(1)
        );
    }

    #[test]
    fn generate_parenthesis_3() {
        assert_eq!(
            sorted(vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
            sorted(generate_parenthesis(3))
        );
    }
}
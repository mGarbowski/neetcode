//! https://leetcode.com/problems/generate-parentheses/

#[derive(Default)]
struct Option {
    combination: Vec<char>,
    n_open: i32,
    n_closed: i32,
}

impl Option {
    pub fn can_add_opening(&self, n: i32) -> bool {
        self.n_open < n
    }

    pub fn can_add_closing(&self, n: i32) -> bool {
        self.n_closed < self.n_open && self.n_closed < n
    }

    pub fn with_opening(&self) -> Self {
        let mut new_combination = self.combination.clone();
        new_combination.push('(');
        Self {
            combination: new_combination,
            n_open: self.n_open + 1,
            n_closed: self.n_closed,
        }
    }

    pub fn with_closing(&self) -> Self {
        let mut new_combination = self.combination.clone();
        new_combination.push(')');
        Self {
            combination: new_combination,
            n_open: self.n_open,
            n_closed: self.n_closed + 1,
        }
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut options = vec![Option::default()];
    for _ in 0..2 * n {
        let mut new_options = vec![];
        for option in options {
            if option.can_add_opening(n) {
                new_options.push(option.with_opening());
            }
            if option.can_add_closing(n) {
                new_options.push(option.with_closing());
            }
        }

        options = new_options
    }


    options.iter()
        .map(|c| c.combination.iter().collect::<String>())
        .collect()
}

pub fn generate_parenthesis_recursive(n: i32) -> Vec<String> {
    fn backtrack(combination: String, n_open_left: i32, n_closed_left: i32) -> Vec<String> {
        let mut combinations = vec![];
        if n_open_left == 0 && n_closed_left == 0 {
            return vec![combination];
        }

        if n_open_left > 0 {
            combinations.append(
                &mut backtrack(combination.clone() + "(", n_open_left - 1, n_closed_left)
            );
        }

        if n_closed_left > 0 && n_closed_left > n_open_left {
            combinations.append(
                &mut backtrack(combination + ")", n_open_left, n_closed_left - 1)
            );
        }

        combinations
    }

    backtrack("".to_string(), n, n)
}


#[cfg(test)]
mod test {
    use crate::*;
    use crate::stack::generate_parentheses::*;

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

    #[test]
    fn generate_parenthesis_recursive_1() {
        assert_eq!(
            vec!["()"],
            generate_parenthesis_recursive(1)
        );
    }

    #[test]
    fn generate_parenthesis_recursive_3() {
        assert_eq!(
            sorted(vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
            sorted(generate_parenthesis_recursive(3))
        );
    }
}
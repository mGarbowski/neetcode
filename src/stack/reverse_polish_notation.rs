//! https://leetcode.com/problems/evaluate-reverse-polish-notation/

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        if token == "+" || token == "-" || token == "*" || token == "/" {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => panic!()
            })
        } else {
            stack.push(token.parse::<i32>().unwrap())
        }
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use crate::stack::reverse_polish_notation::*;

    #[test]
    fn eval_rpn_1() {
        assert_eq!(9, eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ]))
    }

    #[test]
    fn eval_rpn_2() {
        assert_eq!(6, eval_rpn(vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ]))
    }

    #[test]
    fn division() {
        assert_eq!(0, 5 / 10)
    }
}
//! https://leetcode.com/problems/daily-temperatures/

use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Temperature {
    value: i32,
    day: i32
}

impl Temperature {
    fn new(value: i32, day: usize) -> Self {
        Self {value, day: day as i32}
    }
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<Temperature> = vec![];
    let mut next_greater: HashMap<Temperature, Temperature> = HashMap::new();

    for (day, value) in temperatures.iter().enumerate() {
        let current = Temperature::new(*value, day);
        while !stack.is_empty() && stack.last().cloned().unwrap().value < *value {
            let last = stack.pop().unwrap();
            next_greater.insert(last, current);
        }
        stack.push(current);
    }

    let mut result = vec![0; temperatures.len()];
    for (day_idx, value) in temperatures.iter().enumerate() {
        if let Some(next) = next_greater.get(&Temperature::new(*value, day_idx)) {
            result[day_idx] = next.day - day_idx as i32;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::stack::daily_temperatures::*;

    #[test]
    fn daily_temperatures_1() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        )
    }

    #[test]
    fn daily_temperatures_2() {
        assert_eq!(
            daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        )
    }
}

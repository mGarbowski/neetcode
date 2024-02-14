//! https://leetcode.com/problems/daily-temperatures/

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Temperature {
    value: i32,
    day: usize
}

impl Temperature {
    fn new(value: i32, day: usize) -> Self {
        Self {value, day}
    }
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<Temperature> = vec![];
    let mut result: Vec<i32> = vec![0; temperatures.len()];

    for (day, value) in temperatures.iter().enumerate() {
        let current = Temperature::new(*value, day);
        while !stack.is_empty() && stack.last().unwrap().value < *value {
            let last = stack.pop().unwrap();
            result[last.day] = (day - last.day) as i32;
        }
        stack.push(current);
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

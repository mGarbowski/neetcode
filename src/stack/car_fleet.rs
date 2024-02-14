//! https://leetcode.com/problems/car-fleet/

use std::iter::zip;

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut times_to_end: Vec<(i32, f32)> = zip(position, speed)
        .map(|(pos, speed)| (pos, (target - pos) as f32 / speed as f32))
        .collect();
    times_to_end.sort_by_key(|(pos, _)| *pos);

    let mut n_fleets = 0;
    let mut stack: Vec<f32> = times_to_end.into_iter().map(|(_, time)| time).collect();
    while !stack.is_empty() {
        n_fleets += 1;
        let time = stack.pop().unwrap();
        while !stack.last().is_none() && *stack.last().unwrap() <= time {
            stack.pop();
        }
    }
    n_fleets
}

#[cfg(test)]
mod test {
    use crate::stack::car_fleet::*;

    #[test]
    fn car_fleet_1() {
        assert_eq!(3, car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]));
    }

    #[test]
    fn car_fleet_2() {
        assert_eq!(1, car_fleet(10, vec![3], vec![3]));
    }

    #[test]
    fn car_fleet_3() {
        assert_eq!(1, car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]));
    }

    #[test]
    fn car_fleet_4() {
        assert_eq!(2, car_fleet(10, vec![6, 8], vec![3, 2]));
    }
}
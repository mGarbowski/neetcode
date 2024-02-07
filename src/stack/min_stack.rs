//! https://leetcode.com/problems/min-stack/

struct Item {
    value: i32,
    minimum: i32,
}

pub struct MinStack {
    items: Vec<Item>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            items: vec![]
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_min = match self.items.is_empty() || val < self.get_min() {
            true => val,
            false => self.get_min()
        };

        self.items.push(Item { value: val, minimum: new_min })
    }

    pub fn pop(&mut self) {
        self.items.pop();
    }

    pub fn top(&self) -> i32 {
        self.items.last().unwrap().value
    }

    pub fn get_min(&self) -> i32 {
        self.items.last().unwrap().minimum
    }
}

#[cfg(test)]
mod test {
    use crate::stack::min_stack::MinStack;

    #[test]
    fn test_min_stack() {
        let mut min_stack = MinStack::new();

        min_stack.push(-2);
        assert_eq!(-2, min_stack.top());
        assert_eq!(-2, min_stack.get_min());

        min_stack.push(0);
        assert_eq!(0, min_stack.top());
        assert_eq!(-2, min_stack.get_min());

        min_stack.push(-3);
        assert_eq!(-3, min_stack.top());
        assert_eq!(-3, min_stack.get_min());

        min_stack.pop();
        assert_eq!(0, min_stack.top());
        assert_eq!(-2, min_stack.get_min());
    }
}

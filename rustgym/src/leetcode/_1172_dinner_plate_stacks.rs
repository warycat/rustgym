use std::collections::HashMap;

#[derive(Debug)]
struct DinnerPlates {
    capacity: usize,
    stacks: HashMap<usize, Vec<i32>>,
    start: usize,
    end: usize,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let stacks = HashMap::new();
        let start = 0;
        let end = 0;
        DinnerPlates {
            capacity,
            stacks,
            start,
            end,
        }
    }

    fn push(&mut self, val: i32) {
        while self.stacks.entry(self.start).or_default().len() == self.capacity {
            self.start += 1;
        }
        self.stacks.entry(self.start).or_default().push(val);
        self.end = self.end.max(self.start);
    }

    fn pop(&mut self) -> i32 {
        while self.stacks.entry(self.end).or_default().is_empty() {
            if self.end > 0 {
                self.end -= 1;
            } else {
                return -1;
            }
        }
        self.pop_at_stack(self.end as i32)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let i = index as usize;
        if let Some(val) = self.stacks.entry(i).or_default().pop() {
            self.start = self.start.min(i);
            val
        } else {
            -1
        }
    }
}
#[test]
fn test() {
    let mut obj = DinnerPlates::new(2);
    obj.push(1);
    obj.push(2);
    obj.push(3);
    obj.push(4);
    obj.push(5);
    assert_eq!(obj.pop_at_stack(0), 2);
    obj.push(20);
    obj.push(21);
    assert_eq!(obj.pop_at_stack(0), 20);
    assert_eq!(obj.pop_at_stack(2), 21);
    assert_eq!(obj.pop(), 5);
    assert_eq!(obj.pop(), 4);
    assert_eq!(obj.pop(), 3);
    assert_eq!(obj.pop(), 1);
    assert_eq!(obj.pop(), -1);
    let mut obj = DinnerPlates::new(1);
    obj.push(1);
    obj.push(2);
    obj.push(3);
    assert_eq!(obj.pop_at_stack(1), 2);
    assert_eq!(obj.pop(), 3);
    assert_eq!(obj.pop(), 1);
}

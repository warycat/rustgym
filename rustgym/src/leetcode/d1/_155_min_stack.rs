#[derive(Default)]
struct MinStack {
    nums: Vec<i32>,
    mins: Vec<i32>,
    top: i32,
    min: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            nums: vec![],
            mins: vec![],
            top: 0,
            min: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.nums.push(x);
        if let Some(&min) = self.mins.last() {
            self.mins.push(i32::min(x, min));
        } else {
            self.mins.push(x);
        }
        self.set_min();
        self.set_top();
    }

    fn pop(&mut self) {
        self.nums.pop();
        self.mins.pop();
        self.set_min();
        self.set_top();
    }

    fn set_top(&mut self) {
        self.top = if let Some(&last) = self.nums.last() {
            last
        } else {
            0
        }
    }

    fn set_min(&mut self) {
        self.min = if let Some(&last) = self.mins.last() {
            last
        } else {
            0
        }
    }

    fn top(&self) -> i32 {
        self.top
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

#[test]
fn test() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-3);
    assert_eq!(min_stack.get_min(), -3);
    min_stack.pop();
    assert_eq!(min_stack.top(), 0);
    assert_eq!(min_stack.get_min(), -2);
}

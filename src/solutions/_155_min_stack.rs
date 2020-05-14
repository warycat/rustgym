#[derive(Default)]
pub struct MinStack {
    pub nums: Vec<i32>,
    pub mins: Vec<i32>,
    pub top: i32,
    pub min: i32,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            nums: vec![],
            mins: vec![],
            top: 0,
            min: 0,
        }
    }

    pub fn push(&mut self, x: i32) {
        self.nums.push(x);
        if let Some(&min) = self.mins.last() {
            self.mins.push(i32::min(x, min));
        } else {
            self.mins.push(x);
        }
        self.set_min();
        self.set_top();
    }

    pub fn pop(&mut self) {
        self.nums.pop();
        self.mins.pop();
        self.set_min();
        self.set_top();
    }

    pub fn set_top(&mut self) {
        self.top = if let Some(&last) = self.nums.last() {
            last
        } else {
            0
        }
    }

    pub fn set_min(&mut self) {
        self.min = if let Some(&last) = self.mins.last() {
            last
        } else {
            0
        }
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn get_min(&self) -> i32 {
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

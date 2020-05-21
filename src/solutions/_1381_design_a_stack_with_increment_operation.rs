struct CustomStack {
    stack: Vec<i32>,
    inc: Vec<i32>,
    n: usize,
    max_size: usize,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        let max_size = max_size as usize;
        let stack = vec![];
        let inc = vec![0; (1 + max_size) as usize];
        let n = 0;
        CustomStack {
            stack,
            inc,
            n,
            max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.n != self.max_size {
            self.stack.push(x);
            self.n += 1;
        }
    }

    fn pop(&mut self) -> i32 {
        if let Some(mut top) = self.stack.pop() {
            self.inc[self.n - 1] += self.inc[self.n];
            top += self.inc[self.n];
            self.inc[self.n] = 0;
            self.n -= 1;
            top
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = k as usize;
        if k > self.n {
            self.inc[self.n] += val;
        } else {
            self.inc[k] += val;
        }
    }
}

#[test]
fn test() {
    let mut stack = CustomStack::new(3);
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), 2);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.increment(5, 100);
    stack.increment(2, 100);
    assert_eq!(stack.pop(), 103);
    assert_eq!(stack.pop(), 202);
    assert_eq!(stack.pop(), 201);
    assert_eq!(stack.pop(), -1);
}

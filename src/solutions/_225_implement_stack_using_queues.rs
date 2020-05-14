use std::collections::VecDeque;

#[derive(Default)]
struct MyStack {
    queue: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let mut n = self.queue.len();
        self.queue.push_back(x);
        while n > 0 {
            let front = self.queue.pop_front().unwrap();
            self.queue.push_back(front);
            n -= 1;
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[test]
fn test() {
    let mut stack = MyStack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.top(), 2);
    assert_eq!(stack.pop(), 2);
    assert_eq!(stack.empty(), false);
}

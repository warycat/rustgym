#[derive(Default)]
struct MyQueue {
    stack: Vec<i32>,
    temp: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: vec![],
            temp: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        while let Some(x) = self.stack.pop() {
            self.temp.push(x);
        }
        let res = self.temp.pop().unwrap();
        while let Some(x) = self.temp.pop() {
            self.stack.push(x)
        }
        res
    }

    fn peek(&mut self) -> i32 {
        while let Some(x) = self.stack.pop() {
            self.temp.push(x);
        }
        let res = self.temp.pop().unwrap();
        self.stack.push(res);
        while let Some(x) = self.temp.pop() {
            self.stack.push(x)
        }
        res
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[test]
fn test() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.peek(), 1);
    assert_eq!(queue.pop(), 1);
    assert_eq!(queue.empty(), false);
}

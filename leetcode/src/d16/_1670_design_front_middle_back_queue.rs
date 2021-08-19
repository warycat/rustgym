use std::collections::VecDeque;

#[derive(Debug)]
struct FrontMiddleBackQueue {
    front: VecDeque<i32>,
    back: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        let front = VecDeque::new();
        let back = VecDeque::new();
        FrontMiddleBackQueue { front, back }
    }

    fn push_front(&mut self, val: i32) {
        self.front.push_front(val);
        if self.front.len() == self.back.len() + 2 {
            self.back.push_front(self.front.pop_back().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.front.len() == self.back.len() + 1 {
            self.back.push_front(self.front.pop_back().unwrap());
        }
        self.front.push_back(val);
    }

    fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
        if self.front.len() + 1 == self.back.len() {
            self.front.push_back(self.back.pop_front().unwrap());
        }
    }

    fn pop_front(&mut self) -> i32 {
        if let Some(val) = self.front.pop_front() {
            if self.front.len() + 1 == self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            val
        } else {
            -1
        }
    }

    fn pop_middle(&mut self) -> i32 {
        if let Some(val) = self.front.pop_back() {
            if self.front.len() + 1 == self.back.len() {
                self.front.push_back(self.back.pop_front().unwrap());
            }
            val
        } else {
            -1
        }
    }

    fn pop_back(&mut self) -> i32 {
        if let Some(val) = self.back.pop_back() {
            if self.front.len() == self.back.len() + 2 {
                self.back.push_front(self.front.pop_back().unwrap());
            }
            val
        } else {
            if let Some(val) = self.front.pop_back() {
                val
            } else {
                -1
            }
        }
    }
}

#[test]
fn test() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1);
    q.push_back(2);
    q.push_middle(3);
    q.push_middle(4);
    assert_eq!(q.pop_front(), 1);
    assert_eq!(q.pop_middle(), 3);
    assert_eq!(q.pop_middle(), 4);
    assert_eq!(q.pop_back(), 2);
    assert_eq!(q.pop_front(), -1);
}

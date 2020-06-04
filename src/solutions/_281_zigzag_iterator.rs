use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug)]
struct ZigzagIterator {
    queues: VecDeque<VecDeque<i32>>,
}

impl<'a> ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut queues = VecDeque::new();
        if !v1.is_empty() {
            queues.push_back(VecDeque::from_iter(v1));
        }
        if !v2.is_empty() {
            queues.push_back(VecDeque::from_iter(v2));
        }
        ZigzagIterator { queues }
    }

    fn next(&mut self) -> i32 {
        let mut first = self.queues.pop_front().unwrap();
        let res = first.pop_front().unwrap();
        if !first.is_empty() {
            self.queues.push_back(first);
        }
        res
    }

    fn has_next(&self) -> bool {
        !self.queues.is_empty()
    }
}

#[test]
fn test() {
    let v1 = vec![1, 2];
    let v2 = vec![3, 4, 5, 6];
    let res = vec![1, 3, 2, 4, 5, 6];
    let mut obj = ZigzagIterator::new(v1, v2);
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);
}

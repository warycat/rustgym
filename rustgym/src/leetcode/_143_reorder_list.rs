struct Solution;
use rustgym_util::*;
use std::collections::VecDeque;

impl Solution {
    fn reorder_list(head: &mut ListLink) {
        let mut p: ListLink = head.take();
        let mut deque: VecDeque<ListLink> = VecDeque::new();
        while let Some(mut n) = p {
            p = n.next.take();
            deque.push_back(Some(n));
        }
        let mut stack: Vec<ListLink> = vec![];
        let mut direction = true;
        while !deque.is_empty() {
            if direction {
                stack.push(deque.pop_front().unwrap());
            } else {
                stack.push(deque.pop_back().unwrap())
            }
            direction = !direction;
        }
        let mut prev: ListLink = None;
        while let Some(last) = stack.pop() {
            let mut node = last.unwrap();
            node.next = prev;
            prev = Some(node);
        }
        *head = prev
    }
}

#[test]
fn test() {
    let mut head = list!(1, 2, 3, 4, 5);
    Solution::reorder_list(&mut head);
    let res = list!(1, 5, 2, 4, 3);
    assert_eq!(head, res);
}

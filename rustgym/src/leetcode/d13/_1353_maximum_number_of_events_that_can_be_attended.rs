struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn max_events(mut events: Vec<Vec<i32>>) -> i32 {
        events.sort_by_key(|e| e[0]);
        let mut queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let max = events.iter().map(|e| e[1]).max().unwrap();
        let mut it = events.into_iter().peekable();
        let mut res = 0;
        for i in 1..=max {
            while let Some(&front) = queue.peek() {
                if front.0 < i {
                    queue.pop();
                } else {
                    break;
                }
            }
            while let Some(front) = it.peek() {
                if front[0] == i {
                    queue.push(Reverse(front[1]));
                    it.next();
                } else {
                    break;
                }
            }
            if queue.pop().is_some() {
                res += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let events = vec_vec_i32![[1, 2], [2, 3], [3, 4]];
    let res = 3;
    assert_eq!(Solution::max_events(events), res);
    let events = vec_vec_i32![[1, 2], [2, 3], [3, 4], [1, 2]];
    let res = 4;
    assert_eq!(Solution::max_events(events), res);
    let events = vec_vec_i32![[1, 4], [4, 4], [2, 2], [3, 4], [1, 1]];
    let res = 4;
    assert_eq!(Solution::max_events(events), res);
    let events = vec_vec_i32![[1, 100000]];
    let res = 1;
    assert_eq!(Solution::max_events(events), res);
    let events = vec_vec_i32![[1, 1], [1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [1, 7]];
    let res = 7;
    assert_eq!(Solution::max_events(events), res);
}

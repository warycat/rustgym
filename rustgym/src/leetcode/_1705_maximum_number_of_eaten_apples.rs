struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let n = apples.len();
        let mut queue: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();
        let mut i = 0;
        let mut res = 0;
        loop {
            if i < n {
                queue.push(Reverse((i + days[i] as usize, apples[i])));
            }
            if i >= n && queue.is_empty() {
                break;
            }
            while let Some(Reverse((j, left))) = queue.peek() {
                if i >= *j || *left == 0 {
                    queue.pop();
                } else {
                    break;
                }
            }
            if let Some(mut top) = queue.peek_mut() {
                (top.0).1 -= 1;
                res += 1;
            }
            i += 1;
        }
        res
    }
}

#[test]
fn test() {
    let apples = vec![1, 2, 3, 5, 2];
    let days = vec![3, 2, 1, 4, 2];
    let res = 7;
    assert_eq!(Solution::eaten_apples(apples, days), res);
    let apples = vec![3, 0, 0, 0, 0, 2];
    let days = vec![3, 0, 0, 0, 0, 2];
    let res = 5;
    assert_eq!(Solution::eaten_apples(apples, days), res);
}

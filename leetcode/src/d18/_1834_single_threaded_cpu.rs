struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut available: BinaryHeap<(Reverse<i32>, Reverse<usize>)> = BinaryHeap::new();
        let mut stack: Vec<(i32, usize)> = vec![];
        let n = tasks.len();
        for i in 0..n {
            stack.push((tasks[i][0], i));
        }
        stack.sort_unstable();
        stack.reverse();
        let mut res = vec![];
        let mut m = 0;
        let mut t = 0;
        while m < n {
            if let Some(shortest) = available.pop() {
                t += (shortest.0).0;
                m += 1;
                res.push((shortest.1).0 as i32);
            } else {
                t = t.max(stack.last().unwrap().0);
            }
            while let Some(last) = stack.pop() {
                if last.0 <= t {
                    let i = last.1;
                    let earliest = (Reverse(tasks[i][1]), Reverse(i));
                    available.push(earliest);
                } else {
                    stack.push(last);
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let tasks = vec_vec_i32![[1, 2], [2, 4], [3, 2], [4, 1]];
    let res = vec![0, 2, 3, 1];
    assert_eq!(Solution::get_order(tasks), res);
    let tasks = vec_vec_i32![[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]];
    let res = vec![4, 3, 2, 0, 1];
    assert_eq!(Solution::get_order(tasks), res);
}

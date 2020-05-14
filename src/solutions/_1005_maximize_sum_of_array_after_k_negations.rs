struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn largest_sum_after_k_negations(a: Vec<i32>, mut k: i32) -> i32 {
        let reverse: Vec<Reverse<i32>> = a.into_iter().map(Reverse).collect();
        let mut pq = BinaryHeap::from(reverse);
        while k > 0 {
            if let Some(min) = pq.pop() {
                pq.push(Reverse(-min.0));
            }
            k -= 1;
        }
        pq.into_iter().map(|x| x.0).sum()
    }
}

#[test]
fn test() {
    let a = vec![4, 2, 3];
    let k = 1;
    assert_eq!(Solution::largest_sum_after_k_negations(a, k), 5);
    let a = vec![3, -1, 0, 2];
    let k = 3;
    assert_eq!(Solution::largest_sum_after_k_negations(a, k), 6);
    let a = vec![2, -3, -1, 5, -4];
    let k = 2;
    assert_eq!(Solution::largest_sum_after_k_negations(a, k), 13);
}

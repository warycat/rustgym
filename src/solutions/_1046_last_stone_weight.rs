struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut pq: BinaryHeap<i32> = BinaryHeap::from(stones);
        while let Some(a) = pq.pop() {
            if let Some(b) = pq.pop() {
                if a - b != 0 {
                    pq.push(a - b);
                }
            } else {
                return a;
            }
        }
        0
    }
}

#[test]
fn test() {
    let stones = vec![2, 7, 4, 1, 8, 1];
    assert_eq!(Solution::last_stone_weight(stones), 1);
}

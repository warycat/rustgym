struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn connect_sticks(sticks: Vec<i32>) -> i32 {
        let mut queue: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut res = 0;
        for stick in sticks {
            queue.push(Reverse(stick));
        }
        while queue.len() > 1 {
            let x = queue.pop().unwrap().0;
            let y = queue.pop().unwrap().0;
            let z = x + y;
            res += z;
            queue.push(Reverse(z));
        }
        res
    }
}

#[test]
fn test() {
    let sticks = vec![2, 4, 3];
    let res = 14;
    assert_eq!(Solution::connect_sticks(sticks), res);
    let sticks = vec![1, 8, 3, 5];
    let res = 30;
    assert_eq!(Solution::connect_sticks(sticks), res);
}

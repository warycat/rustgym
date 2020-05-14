struct Solution;
use std::collections::BinaryHeap;

type Pair = (i32, usize);

impl Solution {
    fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut pq: BinaryHeap<Pair> = BinaryHeap::new();
        let n = mat.len();
        for i in 0..n {
            let sum = mat[i].iter().sum::<i32>();
            let pair = (sum, i);
            pq.push(pair);
            if pq.len() > k as usize {
                pq.pop();
            }
        }
        let mut res: Vec<i32> = vec![];
        while !pq.is_empty() {
            let biggest = pq.pop().unwrap();
            res.push(biggest.1 as i32);
        }
        res.reverse();
        res
    }
}

#[test]
fn test() {
    let mat: Vec<Vec<i32>> = [
        [1, 1, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 1, 1, 1, 1],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    let k = 3;
    let res = vec![2, 0, 3];
    assert_eq!(Solution::k_weakest_rows(mat, k), res);
}

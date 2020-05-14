struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        for row in matrix {
            for x in row {
                pq.push(x);
                if pq.len() > k as usize {
                    pq.pop();
                }
            }
        }
        pq.pop().unwrap()
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[1, 5, 9], [10, 11, 13], [12, 13, 15]];
    let k = 8;
    let res = 13;
    assert_eq!(Solution::kth_smallest(matrix, k), res);
    let matrix = vec_vec_i32![[1, 2], [1, 3]];
    let k = 3;
    let res = 2;
    assert_eq!(Solution::kth_smallest(matrix, k), res);
}

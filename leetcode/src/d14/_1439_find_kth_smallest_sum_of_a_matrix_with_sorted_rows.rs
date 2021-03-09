struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    fn kth_smallest(mat: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut queue: BinaryHeap<(Reverse<i32>, Vec<usize>)> = BinaryHeap::new();
        let mut visited: HashSet<Vec<usize>> = HashSet::new();
        let mut sum = 0;
        let indexes = vec![0; n];
        for i in 0..n {
            sum += mat[i][0];
        }
        visited.insert(indexes.to_vec());
        queue.push((Reverse(sum), indexes));
        let mut res = 0;
        while k > 0 {
            if let Some((Reverse(sum), indexes)) = queue.pop() {
                res = sum;
                for i in 0..n {
                    let mut next_sum = sum;
                    let mut next_indexes = indexes.to_vec();
                    let j = indexes[i];
                    if j + 1 < m {
                        next_indexes[i] = j + 1;
                        if visited.insert(next_indexes.to_vec()) {
                            next_sum -= mat[i][j];
                            next_sum += mat[i][j + 1];
                            queue.push((Reverse(next_sum), next_indexes));
                        }
                    }
                }
            }
            k -= 1;
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[1, 3, 11], [2, 4, 6]];
    let k = 5;
    let res = 7;
    assert_eq!(Solution::kth_smallest(mat, k), res);
    let mat = vec_vec_i32![[1, 3, 11], [2, 4, 6]];
    let k = 9;
    let res = 17;
    assert_eq!(Solution::kth_smallest(mat, k), res);
    let mat = vec_vec_i32![[1, 10, 10], [1, 4, 5], [2, 3, 6]];
    let k = 7;
    let res = 9;
    assert_eq!(Solution::kth_smallest(mat, k), res);
    let mat = vec_vec_i32![[1, 1, 10], [2, 2, 9]];
    let k = 7;
    let res = 12;
    assert_eq!(Solution::kth_smallest(mat, k), res);
}

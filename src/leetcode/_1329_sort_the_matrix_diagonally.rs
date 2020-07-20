struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat.len();
        let m = mat[0].len();
        let mut hs: HashMap<i32, BinaryHeap<Reverse<i32>>> = HashMap::new();
        for i in 0..n {
            for j in 0..m {
                hs.entry(i as i32 - j as i32)
                    .or_default()
                    .push(Reverse(mat[i][j]));
            }
        }
        for i in 0..n {
            for j in 0..m {
                mat[i][j] = hs.entry(i as i32 - j as i32).or_default().pop().unwrap().0;
            }
        }
        mat
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[3, 3, 1, 1], [2, 2, 1, 2], [1, 1, 1, 2]];
    let res = vec_vec_i32![[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]];
    assert_eq!(Solution::diagonal_sort(mat), res);
}

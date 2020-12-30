struct Solution;
use std::collections::BTreeSet;

impl Solution {
    fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut prefix = vec![vec![0; m + 1]; n];
        for i in 0..n {
            for j in 0..m {
                prefix[i][j + 1] = prefix[i][j] + matrix[i][j];
            }
        }
        let mut res = std::i32::MIN;
        for start in 0..m {
            for end in start + 1..=m {
                let mut bts: BTreeSet<i32> = BTreeSet::new();
                bts.insert(0);
                let mut sum = 0;
                for i in 0..n {
                    sum += prefix[i][end] - prefix[i][start];
                    if let Some(prev) = bts.range(sum - k..).take(1).next() {
                        res = res.max(sum - prev);
                    }
                    bts.insert(sum);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[1, 0, 1], [0, -2, 3]];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_sum_submatrix(matrix, k), res);
}

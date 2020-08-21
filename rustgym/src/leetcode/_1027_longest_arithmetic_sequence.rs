struct Solution;

use std::collections::HashMap;

impl Solution {
    fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = a.len();
        let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new(); n];
        for i in 0..n {
            for j in 0..i {
                let diff = a[i] - a[j];
                let len_j = *dp[j].entry(diff).or_default();
                let len_i = dp[i].entry(diff).or_default();
                *len_i = len_j + 1;
                res = res.max(*len_i);
            }
        }
        res + 1
    }
}

#[test]
fn test() {
    let a = vec![3, 6, 9, 12];
    let res = 4;
    assert_eq!(Solution::longest_arith_seq_length(a), res);
    let a = vec![9, 4, 7, 2, 10];
    let res = 3;
    assert_eq!(Solution::longest_arith_seq_length(a), res);
    let a = vec![20, 1, 15, 3, 10, 5, 8];
    let res = 4;
    assert_eq!(Solution::longest_arith_seq_length(a), res);
}

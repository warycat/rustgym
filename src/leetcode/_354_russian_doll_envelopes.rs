struct Solution;

use std::cmp::Reverse;

impl Solution {
    fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let n = envelopes.len();
        envelopes.sort_unstable_by_key(|v| (v[0], Reverse(v[1])));
        let mut dp = vec![];
        for i in 0..n {
            let height = envelopes[i][1];
            if let Err(p) = dp.binary_search(&height) {
                if p == dp.len() {
                    dp.push(height);
                } else {
                    dp[p] = height;
                }
            }
        }
        dp.len() as i32
    }
}

#[test]
fn test() {
    let envelopes = vec_vec_i32![[5, 4], [6, 4], [6, 7], [2, 3]];
    let res = 3;
    assert_eq!(Solution::max_envelopes(envelopes), res);
    let envelopes = vec_vec_i32![[4, 5], [4, 6], [6, 7], [2, 3], [1, 1]];
    let res = 4;
    assert_eq!(Solution::max_envelopes(envelopes), res);
}

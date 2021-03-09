struct Solution;

use std::cmp::Ordering::*;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn ways_to_distribute(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut memo: Vec<Vec<Option<i64>>> = vec![vec![None; k + 1]; n + 1];
        Self::dp(n, k, &mut memo) as i32
    }

    fn dp(n: usize, k: usize, memo: &mut Vec<Vec<Option<i64>>>) -> i64 {
        match n.cmp(&k) {
            Equal => 1,
            Less => 0,
            Greater => {
                if k == 0 {
                    0
                } else {
                    if let Some(res) = memo[n][k] {
                        res
                    } else {
                        let res = (k as i64 * Self::dp(n - 1, k, memo)
                            + Self::dp(n - 1, k - 1, memo))
                            % MOD;
                        memo[n][k] = Some(res);
                        res
                    }
                }
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let k = 2;
    let res = 3;
    assert_eq!(Solution::ways_to_distribute(n, k), res);
    let n = 4;
    let k = 2;
    let res = 7;
    assert_eq!(Solution::ways_to_distribute(n, k), res);
    let n = 20;
    let k = 5;
    let res = 206085257;
    assert_eq!(Solution::ways_to_distribute(n, k), res);
}

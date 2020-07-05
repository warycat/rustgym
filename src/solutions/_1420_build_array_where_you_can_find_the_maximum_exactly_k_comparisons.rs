struct Solution;
use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mut memo: HashMap<(i32, i32, i32), i64> = HashMap::new();
        Self::dp(n, m, k, &mut memo) as i32
    }

    fn dp(n: i32, m: i32, k: i32, memo: &mut HashMap<(i32, i32, i32), i64>) -> i64 {
        if let Some(&res) = memo.get(&(n, m, k)) {
            return res;
        }
        let res = if k == 0 || m < 1 {
            0
        } else {
            if n == 1 {
                if k == 1 {
                    m as i64
                } else {
                    0
                }
            } else {
                (Self::dp(n, m - 1, k, memo)
                    + Self::dp(n - 1, m - 1, k - 1, memo)
                    + (Self::dp(n - 1, m, k, memo) + MOD - Self::dp(n - 1, m - 1, k, memo))
                        * m as i64)
                    % MOD
            }
        };
        memo.insert((n, m, k), res);
        res
    }
}

#[test]
fn test() {
    let n = 2;
    let m = 3;
    let k = 1;
    let res = 6;
    assert_eq!(Solution::num_of_arrays(n, m, k), res);
    let n = 5;
    let m = 2;
    let k = 3;
    let res = 0;
    assert_eq!(Solution::num_of_arrays(n, m, k), res);
    let n = 9;
    let m = 1;
    let k = 1;
    let res = 1;
    assert_eq!(Solution::num_of_arrays(n, m, k), res);
    let n = 50;
    let m = 100;
    let k = 25;
    let res = 34549172;
    assert_eq!(Solution::num_of_arrays(n, m, k), res);
    let n = 37;
    let m = 17;
    let k = 7;
    let res = 418930126;
    assert_eq!(Solution::num_of_arrays(n, m, k), res);
}

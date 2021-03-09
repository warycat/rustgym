struct Solution;

use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn number_of_sets(n: i32, k: i32) -> i32 {
        let mut memo: HashMap<(i32, i32, bool), i64> = HashMap::new();
        Self::dp(n, k, false, &mut memo) as i32
    }

    fn dp(n: i32, k: i32, started: bool, memo: &mut HashMap<(i32, i32, bool), i64>) -> i64 {
        if k == 0 {
            return 1;
        }
        if n == 0 {
            return 0;
        }
        if let Some(&res) = memo.get(&(n, k, started)) {
            return res;
        }
        let mut res = Self::dp(n - 1, k, started, memo);
        if started {
            res += Self::dp(n, k - 1, false, memo);
        } else {
            res += Self::dp(n - 1, k, true, memo);
        }
        res %= MOD;
        memo.insert((n, k, started), res);
        res
    }
}

#[test]
fn test() {
    let n = 4;
    let k = 2;
    let res = 5;
    assert_eq!(Solution::number_of_sets(n, k), res);
    let n = 3;
    let k = 1;
    let res = 3;
    assert_eq!(Solution::number_of_sets(n, k), res);
    let n = 30;
    let k = 7;
    let res = 796297179;
    assert_eq!(Solution::number_of_sets(n, k), res);
}

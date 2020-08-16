struct Solution;

use std::collections::HashMap;

impl Solution {
    fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let mut memo: HashMap<(i32, i32), i32> = HashMap::new();
        Self::dp(n, m, &mut memo)
    }
    fn dp(n: i32, m: i32, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
        if n == m {
            1
        } else {
            if let Some(&res) = memo.get(&(n, m)) {
                return res;
            }
            let mut res = std::i32::MAX;

            for i in 1..n {
                res = res.min(Self::dp(i, m, memo) + Self::dp(n - i, m, memo));
            }

            for j in 1..m {
                res = res.min(Self::dp(n, j, memo) + Self::dp(n, m - j, memo));
            }

            for i in 1..n - 1 {
                for j in 1..m - 1 {
                    let a = Self::dp(i, j + 1, memo);
                    let b = Self::dp(i + 1, m - j - 1, memo);
                    let c = Self::dp(n - i, j, memo);
                    let d = Self::dp(n - i - 1, m - j, memo);
                    res = res.min(a + b + c + d + 1);
                }
            }
            memo.insert((n, m), res);
            res
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let m = 3;
    let res = 3;
    assert_eq!(Solution::tiling_rectangle(n, m), res);
    let n = 5;
    let m = 8;
    let res = 5;
    assert_eq!(Solution::tiling_rectangle(n, m), res);
    let n = 11;
    let m = 13;
    let res = 6;
    assert_eq!(Solution::tiling_rectangle(n, m), res);
    let n = 7;
    let m = 6;
    let res = 5;
    assert_eq!(Solution::tiling_rectangle(n, m), res);
    let n = 10;
    let m = 9;
    let res = 6;
    assert_eq!(Solution::tiling_rectangle(n, m), res);
}

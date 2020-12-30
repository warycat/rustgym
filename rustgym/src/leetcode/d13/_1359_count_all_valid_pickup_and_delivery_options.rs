struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn count_orders(n: i32) -> i32 {
        let n = n as i64;
        let mut res: i64 = 1;
        for i in 1..=n {
            res *= i * 2 - 1;
            res %= MOD;
            res *= i;
            res %= MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 1;
    assert_eq!(Solution::count_orders(n), res);
    let n = 2;
    let res = 6;
    assert_eq!(Solution::count_orders(n), res);
    let n = 3;
    let res = 90;
    assert_eq!(Solution::count_orders(n), res);
}

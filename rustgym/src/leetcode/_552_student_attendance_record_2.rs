struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn check_record(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 3;
        }
        let mut p = vec![0; n + 1];
        let mut l = vec![0; n + 1];
        p[1] = 1;
        l[1] = 1;
        p[2] = 2;
        l[2] = 2;
        for i in 3..=n {
            p[i] = l[i - 1] + p[i - 1];
            p[i] %= MOD;
            l[i] = p[i - 1] + p[i - 2];
            l[i] %= MOD;
        }
        let mut lp = vec![0; n + 1];
        lp[0] = 1;
        for i in 1..=n {
            lp[i] = l[i] + p[i];
            lp[i] %= MOD;
        }

        let mut res: i64 = lp[n];
        for i in 0..n {
            res += (lp[i]) * (lp[n - 1 - i]);
            res %= MOD;
        }
        res as i32
    }
}

#[test]
fn test() {
    let n = 2;
    let res = 8;
    assert_eq!(Solution::check_record(n), res);
}

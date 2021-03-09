struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn sum_subseq_widths(mut a: Vec<i32>) -> i32 {
        let n = a.len();
        a.sort_unstable();
        let mut res = 0;
        let mut c = 1;
        for i in 0..n {
            res += c * a[i] as i64;
            res %= MOD;
            c *= 2;
            c %= MOD;
        }
        c = 1;
        for i in (0..n).rev() {
            res -= c * a[i] as i64;
            res %= MOD;
            c *= 2;
            c %= MOD;
        }
        ((res + MOD) % MOD) as i32
    }
}

#[test]
fn test() {
    let a = vec![2, 1, 3];
    let res = 6;
    assert_eq!(Solution::sum_subseq_widths(a), res);
}

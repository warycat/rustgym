struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn count_vowel_permutation(n: i32) -> i32 {
        let n = n as usize;
        let mut a = vec![0; n];
        let mut e = vec![0; n];
        let mut i = vec![0; n];
        let mut o = vec![0; n];
        let mut u = vec![0; n];
        a[0] = 1;
        e[0] = 1;
        i[0] = 1;
        o[0] = 1;
        u[0] = 1;
        for j in 1..n {
            e[j] += a[j - 1];

            a[j] += e[j - 1];
            i[j] += e[j - 1];

            a[j] += i[j - 1];
            e[j] += i[j - 1];
            o[j] += i[j - 1];
            u[j] += i[j - 1];

            i[j] += o[j - 1];
            u[j] += o[j - 1];

            a[j] += u[j - 1];

            a[j] %= MOD;
            e[j] %= MOD;
            i[j] %= MOD;
            o[j] %= MOD;
            u[j] %= MOD;
        }
        ((a[n - 1] + e[n - 1] + i[n - 1] + o[n - 1] + u[n - 1]) % MOD) as i32
    }
}

#[test]
fn test() {
    let n = 1;
    let res = 5;
    assert_eq!(Solution::count_vowel_permutation(n), res);
    let n = 2;
    let res = 10;
    assert_eq!(Solution::count_vowel_permutation(n), res);
    let n = 5;
    let res = 68;
    assert_eq!(Solution::count_vowel_permutation(n), res);
}

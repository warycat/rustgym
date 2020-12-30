struct Solution;

const MOD: usize = 1_000_000_007;

impl Solution {
    fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        if n == 0 {
            return 0;
        }
        let mut memo: Vec<usize> = vec![0; n + 1];
        Self::dp(n, &mut memo) as i32
    }
    fn dp(n: usize, memo: &mut Vec<usize>) -> usize {
        match n {
            0 => 1,
            1 => 1,
            2 => 2,
            3 => 5,
            _ => {
                if memo[n] > 0 {
                    return memo[n];
                }
                let mut res = 0;
                res += Self::dp(n - 3, memo);
                res %= MOD;
                res += 2 * Self::dp(n - 1, memo);
                res %= MOD;
                memo[n] = res;
                res
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let res = 5;
    assert_eq!(Solution::num_tilings(n), res);
    let n = 4;
    let res = 11;
    assert_eq!(Solution::num_tilings(n), res);
}

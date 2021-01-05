struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    fn num_perms_di_sequence(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut memo = vec![vec![-1; n + 1]; n + 1];
        for j in 0..=n {
            memo[n][j] = 1;
        }
        Self::dp(0, 0, &mut memo, &s, n) as i32
    }

    fn dp(i: usize, j: usize, memo: &mut Vec<Vec<i64>>, s: &[char], n: usize) -> i64 {
        if memo[i][j] != -1 {
            memo[i][j]
        } else {
            let mut res = 0;
            if s[i] == 'I' {
                for k in j + 1..=i + 1 {
                    res += Self::dp(i + 1, k, memo, s, n);
                    res %= MOD;
                }
            }
            if s[i] == 'D' {
                for k in 0..=j {
                    res += Self::dp(i + 1, k, memo, s, n);
                    res %= MOD;
                }
            }
            memo[i][j] = res;
            res
        }
    }
}

#[test]
fn test() {
    let s = "DID".to_string();
    let res = 5;
    assert_eq!(Solution::num_perms_di_sequence(s), res);
    let s = "D".to_string();
    let res = 1;
    assert_eq!(Solution::num_perms_di_sequence(s), res);
}

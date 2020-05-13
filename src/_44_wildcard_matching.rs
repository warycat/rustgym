struct Solution;

impl Solution {
    fn is_match(s: String, p: String) -> bool {
        let n = s.len();
        let m = p.len();
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; m + 1]; n + 1];
        Self::is_match_dp(n, m, &mut memo, &s, &p)
    }

    fn is_match_dp(
        n: usize,
        m: usize,
        memo: &mut Vec<Vec<Option<bool>>>,
        s: &[char],
        p: &[char],
    ) -> bool {
        if let Some(ans) = memo[n][m] {
            ans
        } else {
            let res = if n == 0 && m == 0 {
                true
            } else if n == 0 && m != 0 {
                if p[m - 1] == '*' {
                    Self::is_match_dp(n, m - 1, memo, s, p)
                } else {
                    false
                }
            } else if n != 0 && m == 0 {
                false
            } else {
                if s[n - 1] == p[m - 1] {
                    Self::is_match_dp(n - 1, m - 1, memo, s, p)
                } else {
                    match p[m - 1] {
                        '?' => Self::is_match_dp(n - 1, m - 1, memo, s, p),
                        '*' => {
                            Self::is_match_dp(n - 1, m, memo, s, p)
                                || Self::is_match_dp(n, m - 1, memo, s, p)
                        }
                        _ => false,
                    }
                }
            };

            memo[n][m] = Some(res);
            res
        }
    }
}

#[test]
fn test() {
    let s = "aa".to_string();
    let p = "a".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "aa".to_string();
    let p = "*".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "cb".to_string();
    let p = "?a".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "adceb".to_string();
    let p = "*a*b".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "acdcb".to_string();
    let p = "a*c?b".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
}

struct Solution;

impl Solution {
    fn min_window(s: String, t: String) -> String {
        let m = t.len();
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];
        for j in 0..=n {
            dp[0][j] = j + 1;
        }
        for i in 1..=m {
            for j in 1..=n {
                if t[i - 1] == s[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
        let mut start = 0;
        let mut len = std::usize::MAX;
        for j in 1..=n {
            if dp[m][j] != 0 {
                if j - dp[m][j] + 1 < len {
                    start = dp[m][j] - 1;
                    len = j - dp[m][j] + 1;
                }
            }
        }
        if len == std::usize::MAX {
            "".to_string()
        } else {
            s[start..start + len].iter().collect()
        }
    }
}

#[test]
fn test() {
    // let s = "abcdebdde".to_string();
    // let t = "bde".to_string();
    // let res = "bcde".to_string();
    // assert_eq!(Solution::min_window(s, t), res);
    let s = "jmeqksfrsdcmsiwvaovztaqenprpvnbstl".to_string();
    let t = "l".to_string();
    let res = "l".to_string();
    assert_eq!(Solution::min_window(s, t), res);
}

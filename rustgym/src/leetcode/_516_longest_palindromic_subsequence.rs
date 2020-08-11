struct Solution;
use std::i32;

impl Solution {
    fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; n];
        for i in 0..n {
            dp[i][i + 1] = 1;
        }
        for w in 2..=n {
            for i in 0..=n - w {
                let j = i + w;
                if s[i] == s[j - 1] {
                    dp[i][j] = 2 + dp[i + 1][j - 1];
                } else {
                    dp[i][j] = i32::max(dp[i + 1][j], dp[i][j - 1]);
                }
            }
        }
        dp[0][n]
    }
}

#[test]
fn test() {
    let s = "bbbab".to_string();
    let res = 4;
    assert_eq!(Solution::longest_palindrome_subseq(s), res);
    let s = "cbbd".to_string();
    let res = 2;
    assert_eq!(Solution::longest_palindrome_subseq(s), res);
}

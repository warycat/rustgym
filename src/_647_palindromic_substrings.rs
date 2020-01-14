struct Solution;

impl Solution {
    fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut res = 0;
        let mut dp = vec![vec![false; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in i..n {
                if j == i
                    || j == i + 1 && s[i] == s[j]
                    || j > i + 1 && s[i] == s[j] && dp[i + 1][j - 1]
                {
                    dp[i][j] = true;
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let s = "abc".to_string();
    let res = 3;
    assert_eq!(Solution::count_substrings(s), res);
    let s = "aaa".to_string();
    let res = 6;
    assert_eq!(Solution::count_substrings(s), res);
}

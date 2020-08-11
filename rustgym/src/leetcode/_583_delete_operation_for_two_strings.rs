struct Solution;

impl Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let w1: Vec<char> = word1.chars().collect();
        let w2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                if w1[i] == w2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }
        (n + m - dp[n][m] * 2) as i32
    }
}

#[test]
fn test() {
    let word1 = "sea".to_string();
    let word2 = "eat".to_string();
    let res = 2;
    assert_eq!(Solution::min_distance(word1, word2), res);
}

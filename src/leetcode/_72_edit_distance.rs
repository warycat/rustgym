struct Solution;

impl Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = i;
        }
        for j in 0..=m {
            dp[0][j] = j;
        }
        for i in 1..=n {
            for j in 1..=m {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }
        dp[n][m] as i32
    }
}

#[test]
fn test() {
    let word1 = "horse".to_string();
    let word2 = "ros".to_string();
    let res = 3;
    assert_eq!(Solution::min_distance(word1, word2), res);
    let word1 = "intention".to_string();
    let word2 = "execution".to_string();
    let res = 5;
    assert_eq!(Solution::min_distance(word1, word2), res);
}

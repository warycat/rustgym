struct Solution;

impl Solution {
    fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<u8> = s1.bytes().collect();
        let s2: Vec<u8> = s2.bytes().collect();
        let sum: i32 = s1.iter().chain(s2.iter()).map(|&b| b as i32).sum();
        let n = s1.len();
        let m = s2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + s1[i] as i32 + s2[j] as i32;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }
        sum - dp[n][m]
    }
}

#[test]
fn test() {
    let s1 = "sea".to_string();
    let s2 = "eat".to_string();
    let res = 231;
    assert_eq!(Solution::minimum_delete_sum(s1, s2), res);
    let s1 = "delete".to_string();
    let s2 = "leet".to_string();
    let res = 403;
    assert_eq!(Solution::minimum_delete_sum(s1, s2), res);
}

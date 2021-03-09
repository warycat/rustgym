struct Solution;

impl Solution {
    fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1: Vec<char> = str1.chars().collect();
        let s2: Vec<char> = str2.chars().collect();
        let n = s1.len();
        let m = s2.len();
        let mut dp = vec![vec![(' ', 0, std::usize::MAX, std::usize::MAX); m + 1]; n + 1];
        for i in 0..n {
            dp[i + 1][0] = (s1[i], i + 1, i, 0);
        }
        for j in 0..m {
            dp[0][j + 1] = (s2[j], j + 1, 0, j);
        }
        for i in 0..n {
            for j in 0..m {
                if s1[i] == s2[j] {
                    dp[i + 1][j + 1] = (s1[i], dp[i][j].1 + 1, i, j);
                } else {
                    if dp[i][j + 1].1 < dp[i + 1][j].1 {
                        dp[i + 1][j + 1] = (s1[i], dp[i][j + 1].1 + 1, i, j + 1);
                    } else {
                        dp[i + 1][j + 1] = (s2[j], dp[i + 1][j].1 + 1, i + 1, j);
                    }
                }
            }
        }
        let mut path = vec![];
        let mut i = n;
        let mut j = m;
        while dp[i][j].0 != ' ' {
            path.push(dp[i][j].0);
            let next = dp[i][j];
            i = next.2;
            j = next.3;
        }
        path.into_iter().rev().collect()
    }
}

#[test]
fn test() {
    let str1 = "abac".to_string();
    let str2 = "cab".to_string();
    let res = "cabac".to_string();
    assert_eq!(Solution::shortest_common_supersequence(str1, str2), res);
}

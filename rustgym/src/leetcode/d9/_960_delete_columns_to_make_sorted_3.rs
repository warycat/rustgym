struct Solution;

impl Solution {
    fn min_deletion_size(a: Vec<String>) -> i32 {
        let n = a.len();
        let a: Vec<Vec<char>> = a.into_iter().map(|s| s.chars().collect()).collect();
        let m = a[0].len();
        let mut dp = vec![1; m];
        for i in 1..m {
            'outer: for j in 0..i {
                for k in 0..n {
                    if a[k][j] > a[k][i] {
                        continue 'outer;
                    }
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
        (m - dp.into_iter().max().unwrap()) as i32
    }
}

#[test]
fn test() {
    let a = vec_string!["babca", "bbazb"];
    let res = 3;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec_string!["edcba"];
    let res = 4;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec_string!["ghi", "def", "abc"];
    let res = 0;
    assert_eq!(Solution::min_deletion_size(a), res);
    let a = vec_string!["cbbdabc"];
    let res = 3;
    assert_eq!(Solution::min_deletion_size(a), res);
}

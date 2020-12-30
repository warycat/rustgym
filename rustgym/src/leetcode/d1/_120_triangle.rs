struct Solution;

impl Solution {
    fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![vec![0; n + 1]; 2];
        for i in (0..n).rev() {
            for j in 0..=i {
                dp[i % 2][j] = triangle[i][j] + dp[(i + 1) % 2][j].min(dp[(i + 1) % 2][j + 1]);
            }
        }
        dp[0][0]
    }
}

#[test]
fn test() {
    let triangle = vec_vec_i32![[2], [3, 4], [6, 5, 7], [4, 1, 8, 3]];
    let res = 11;
    assert_eq!(Solution::minimum_total(triangle), res);
}

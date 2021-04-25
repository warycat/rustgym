struct Solution;

impl Solution {
    fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0, 0, 0]; n];
        dp[0][0] = 1;
        dp[0][2] = 1;
        for i in 1..n {
            for j in 0..3 {
                if obstacles[i] == (j + 1) as i32 {
                    dp[i][j] = std::i32::MAX;
                } else {
                    let mut min = std::i32::MAX;
                    for k in 0..3 {
                        if !(obstacles[i - 1] == (k + 1) as i32 || obstacles[i] == (k + 1) as i32) {
                            min = min.min(dp[i - 1][k] + if k == j { 0 } else { 1 });
                        }
                    }
                    dp[i][j] = min;
                }
            }
        }
        *dp[n - 1].iter().min().unwrap()
    }
}

#[test]
fn test() {
    let obstacles = vec![0, 1, 2, 3, 0];
    let res = 2;
    assert_eq!(Solution::min_side_jumps(obstacles), res);
    let obstacles = vec![0, 1, 1, 3, 3, 0];
    let res = 0;
    assert_eq!(Solution::min_side_jumps(obstacles), res);
    let obstacles = vec![0, 2, 1, 0, 3, 0];
    let res = 2;
    assert_eq!(Solution::min_side_jumps(obstacles), res);
}

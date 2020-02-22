struct Solution;

impl Solution {
    fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let mut res = 0;
        let mut dp = vec![vec![0; m]; m];
        for row in grid {
            for l in 0..m - 1 {
                if row[l] == 1 {
                    for r in l + 1..m {
                        if row[r] == 1 {
                            res += dp[l][r];
                            dp[l][r] += 1;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![
        [1, 0, 0, 1, 0],
        [0, 0, 1, 0, 1],
        [0, 0, 0, 1, 0],
        [1, 0, 1, 0, 1]
    ];
    let res = 1;
    assert_eq!(Solution::count_corner_rectangles(grid), res);
    let grid = vec_vec_i32![[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let res = 9;
    assert_eq!(Solution::count_corner_rectangles(grid), res);
    let grid = vec_vec_i32![[1, 1, 1, 1]];
    let res = 0;
    assert_eq!(Solution::count_corner_rectangles(grid), res);
}

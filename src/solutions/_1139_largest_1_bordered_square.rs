struct Solution;

impl Solution {
    fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut top = vec![vec![0; m]; n];
        let mut down = vec![vec![0; m]; n];
        let mut left = vec![vec![0; m]; n];
        let mut right = vec![vec![0; m]; n];
        let mut res = 0;

        for i in 0..n {
            for j in 0..m {
                top[i][j] = if grid[i][j] == 1 {
                    1 + if i > 0 { top[i - 1][j] } else { 0 }
                } else {
                    0
                };
                left[i][j] = if grid[i][j] == 1 {
                    1 + if j > 0 { left[i][j - 1] } else { 0 }
                } else {
                    0
                };
            }
        }

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                down[i][j] = if grid[i][j] == 1 {
                    1 + if i + 1 < n { down[i + 1][j] } else { 0 }
                } else {
                    0
                };
                right[i][j] = if grid[i][j] == 1 {
                    1 + if j + 1 < m { right[i][j + 1] } else { 0 }
                } else {
                    0
                };
            }
        }
        for i in 0..n {
            for j in 0..m {
                for k in 1..=(n - i).min(m - j) {
                    if top[i + k - 1][j + k - 1] >= k
                        && down[i][j] >= k
                        && left[i + k - 1][j + k - 1] >= k
                        && right[i][j] >= k
                    {
                        res = res.max(k);
                    }
                }
            }
        }
        (res * res) as i32
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let res = 9;
    assert_eq!(Solution::largest1_bordered_square(grid), res);
    let grid = vec_vec_i32![[1, 1, 0, 0]];
    let res = 1;
    assert_eq!(Solution::largest1_bordered_square(grid), res);
}

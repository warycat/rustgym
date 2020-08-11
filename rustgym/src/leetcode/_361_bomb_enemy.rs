struct Solution;

impl Solution {
    fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut left = vec![vec![0; m]; n];
        let mut top = vec![vec![0; m]; n];
        let mut bottom = vec![vec![0; m]; n];
        let mut right = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    'E' => {
                        top[i][j] = if i > 0 { top[i - 1][j] } else { 0 } + 1;
                        left[i][j] = if j > 0 { left[i][j - 1] } else { 0 } + 1;
                    }
                    'W' => {
                        top[i][j] = 0;
                        left[i][j] = 0;
                    }
                    _ => {
                        top[i][j] = if i > 0 { top[i - 1][j] } else { 0 };
                        left[i][j] = if j > 0 { left[i][j - 1] } else { 0 };
                    }
                }
            }
        }
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                match grid[i][j] {
                    'E' => {
                        bottom[i][j] = if i + 1 < n { bottom[i + 1][j] } else { 0 } + 1;
                        right[i][j] = if j + 1 < m { right[i][j + 1] } else { 0 } + 1;
                    }
                    'W' => {
                        bottom[i][j] = 0;
                        right[i][j] = 0;
                    }
                    _ => {
                        bottom[i][j] = if i + 1 < n { bottom[i + 1][j] } else { 0 };
                        right[i][j] = if j + 1 < m { right[i][j + 1] } else { 0 };
                    }
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '0' {
                    res = res.max(top[i][j] + left[i][j] + bottom[i][j] + right[i][j]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec_vec_char![
        ['0', 'E', '0', '0'],
        ['E', '0', 'W', 'E'],
        ['0', 'E', '0', '0']
    ];
    let res = 3;
    assert_eq!(Solution::max_killed_enemies(grid), res);
}

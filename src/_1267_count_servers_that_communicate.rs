struct Solution;

impl Solution {
    fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1) {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 0], [0, 1]];
    let res = 0;
    assert_eq!(Solution::count_servers(grid), res);
    let grid = vec_vec_i32![[1, 0], [1, 1]];
    let res = 3;
    assert_eq!(Solution::count_servers(grid), res);
    let grid = vec_vec_i32![[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    let res = 4;
    assert_eq!(Solution::count_servers(grid), res);
}

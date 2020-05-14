struct Solution;

impl Solution {
    fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut row: Vec<i32> = vec![0; n];
        let mut col: Vec<i32> = vec![0; m];
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                row[i] = row[i].max(grid[i][j]);
                col[j] = col[j].max(grid[i][j]);
            }
        }
        for i in 0..n {
            for j in 0..m {
                res += row[i].min(col[j]) - grid[i][j];
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]];
    let res = 35;
    assert_eq!(Solution::max_increase_keeping_skyline(grid), res);
}

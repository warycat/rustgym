struct Solution;

impl Solution {
    fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut sum_z: i32 = 0;
        let n = grid.len();
        let m = grid[0].len();
        let mut x = vec![0; n];
        let mut y = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] != 0 {
                    sum_z += 1;
                }
                x[i] = i32::max(x[i], grid[i][j]);
                y[j] = i32::max(y[j], grid[i][j]);
            }
        }

        let sum_x: i32 = x.iter().sum();
        let sum_y: i32 = y.iter().sum();
        sum_x + sum_y + sum_z
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2]];
    let res = 5;
    assert_eq!(Solution::projection_area(grid), res);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [3, 4]];
    let res = 17;
    assert_eq!(Solution::projection_area(grid), res);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [0, 2]];
    let res = 8;
    assert_eq!(Solution::projection_area(grid), res);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let res = 14;
    assert_eq!(Solution::projection_area(grid), res);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2, 2, 2], [2, 1, 2], [2, 2, 2]];
    let res = 21;
    assert_eq!(Solution::projection_area(grid), res);
}

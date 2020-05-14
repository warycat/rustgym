struct Solution;

impl Solution {
    fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] > 0 {
                    res += 2 + 4 * grid[i][j];
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                if i > 0 {
                    res -= 2 * i32::min(grid[i][j], grid[i - 1][j]);
                }
                if j > 0 {
                    res -= 2 * i32::min(grid[i][j], grid[i][j - 1]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2]];
    assert_eq!(Solution::surface_area(grid), 10);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [3, 4]];
    assert_eq!(Solution::surface_area(grid), 34);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 0], [0, 2]];
    assert_eq!(Solution::surface_area(grid), 16);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    assert_eq!(Solution::surface_area(grid), 32);
    let grid: Vec<Vec<i32>> = vec_vec_i32![[2, 2, 2], [2, 1, 2], [2, 2, 2]];
    assert_eq!(Solution::surface_area(grid), 46);
}

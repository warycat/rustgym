struct Solution;

impl Solution {
    fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut j = m;
        let mut res = 0;
        for i in 0..n {
            while j > 0 && grid[i][j - 1] < 0 {
                j -= 1;
            }
            res += m - j;
        }
        res as i32
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![
        [4, 3, 2, -1],
        [3, 2, 1, -1],
        [1, 1, -1, -2],
        [-1, -1, -2, -3]
    ];
    let res = 8;
    assert_eq!(Solution::count_negatives(grid), res);
    let grid = vec_vec_i32![[3, 2], [1, 0]];
    let res = 0;
    assert_eq!(Solution::count_negatives(grid), res);
    let grid = vec_vec_i32![[1, -1], [-1, -1]];
    let res = 3;
    assert_eq!(Solution::count_negatives(grid), res);
    let grid = vec_vec_i32![[-1]];
    let res = 1;
    assert_eq!(Solution::count_negatives(grid), res);
}

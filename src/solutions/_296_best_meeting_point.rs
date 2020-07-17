struct Solution;

impl Solution {
    fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut rows = vec![];
        let mut cols = vec![];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    rows.push(i as i32);
                }
            }
        }
        for j in 0..m {
            for i in 0..n {
                if grid[i][j] == 1 {
                    cols.push(j as i32);
                }
            }
        }
        let r = Self::median(&rows);
        let c = Self::median(&cols);
        Self::distance(&rows, r) + Self::distance(&cols, c)
    }

    fn median(values: &[i32]) -> i32 {
        let n = values.len();
        values[n / 2]
    }

    fn distance(values: &[i32], center: i32) -> i32 {
        values.iter().map(|&v| (v - center).abs()).sum()
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[1, 0, 0, 0, 1], [0, 0, 0, 0, 0], [0, 0, 1, 0, 0]];
    let res = 6;
    assert_eq!(Solution::min_total_distance(grid), res);
}

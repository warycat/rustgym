struct Solution;

impl Solution {
    fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                res = i32::max(Self::area(&mut grid, n, m, i, j), res);
            }
        }
        res
    }

    fn area(grid: &mut Vec<Vec<i32>>, n: usize, m: usize, row: usize, col: usize) -> i32 {
        if grid[row][col] <= 0 {
            0
        } else {
            grid[row][col] *= -1;
            let mut sum = 1;
            if row > 0 {
                sum += Self::area(grid, n, m, row - 1, col);
            }
            if row + 1 < n {
                sum += Self::area(grid, n, m, row + 1, col);
            }
            if col > 0 {
                sum += Self::area(grid, n, m, row, col - 1);
            }
            if col + 1 < m {
                sum += Self::area(grid, n, m, row, col + 1);
            }
            sum
        }
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = [
        [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ]
    .iter()
    .map(|v| v.to_vec())
    .collect();
    assert_eq!(Solution::max_area_of_island(grid), 6);
}

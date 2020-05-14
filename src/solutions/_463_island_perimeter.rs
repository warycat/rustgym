struct Solution;

impl Solution {
    fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut sum = 0;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    if i > 0 && grid[i - 1][j] == 0 || i == 0 {
                        sum += 1;
                    }
                    if i < n - 1 && grid[i + 1][j] == 0 || i == n - 1 {
                        sum += 1;
                    }
                    if j > 0 && grid[i][j - 1] == 0 || j == 0 {
                        sum += 1;
                    }
                    if j < m - 1 && grid[i][j + 1] == 0 || j == m - 1 {
                        sum += 1;
                    }
                }
            }
        }
        sum
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    assert_eq!(Solution::island_perimeter(grid), 16);
}

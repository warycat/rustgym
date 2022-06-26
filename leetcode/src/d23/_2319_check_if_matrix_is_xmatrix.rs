struct Solution;

impl Solution {
    pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
        let n = grid.len();
        for i in 0..n {
            for j in 0..n {
                if (i == j || i == n - j - 1) {
                    if grid[i][j] == 0 {
                        return false;
                    }
                } else {
                    if grid[i][j] != 0 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::check_x_matrix(vec_vec_i32![
            [2, 0, 0, 1],
            [0, 3, 1, 0],
            [0, 5, 2, 0],
            [4, 0, 0, 2]
        ]),
        true
    );
    assert_eq!(
        Solution::check_x_matrix(vec_vec_i32![[5, 7, 0], [0, 3, 1], [0, 5, 0]]),
        false
    );
}

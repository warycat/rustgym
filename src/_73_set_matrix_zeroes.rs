struct Solution;
use std::collections::HashSet;

impl Solution {
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut row: HashSet<usize> = HashSet::new();
        let mut col: HashSet<usize> = HashSet::new();
        let n = matrix.len();
        let m = matrix[0].len();
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    row.insert(i);
                    col.insert(j);
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if row.contains(&i) || col.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[1, 0, 1], [0, 0, 0], [1, 0, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, res);
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32![[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, res);
}

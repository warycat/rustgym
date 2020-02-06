struct Solution;

impl Solution {
    fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        for i in 0..n - 1 {
            for j in 0..m - 1 {
                if matrix[i][j] != matrix[i + 1][j + 1] {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]];
    assert_eq!(Solution::is_toeplitz_matrix(matrix), true);
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2], [2, 2]];
    assert_eq!(Solution::is_toeplitz_matrix(matrix), false);
}

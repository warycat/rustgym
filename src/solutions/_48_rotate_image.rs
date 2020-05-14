struct Solution;

impl Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let a = matrix[i][j];
                let b = matrix[j][i];
                matrix[i][j] = b;
                matrix[j][i] = a;
            }
        }
    }
}

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res: Vec<Vec<i32>> = vec_vec_i32![[7, 4, 1], [8, 5, 2], [9, 6, 3]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, res);
}

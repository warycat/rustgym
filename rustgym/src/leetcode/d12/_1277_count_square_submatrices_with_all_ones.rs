struct Solution;

impl Solution {
    fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 1 {
                    matrix[i][j] = if i > 0 && j > 0 {
                        1 + [matrix[i - 1][j], matrix[i][j - 1], matrix[i - 1][j - 1]]
                            .iter()
                            .min()
                            .unwrap()
                    } else {
                        1
                    };
                }
                res += matrix[i][j];
            }
        }
        res
    }
}
#[test]
fn test() {
    let matrix = vec_vec_i32![[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]];
    let res = 15;
    assert_eq!(Solution::count_squares(matrix), res);
    let matrix = vec_vec_i32![[1, 0, 1], [1, 1, 0], [1, 1, 0]];
    let res = 7;
    assert_eq!(Solution::count_squares(matrix), res);
}

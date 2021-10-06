struct Solution;

impl Solution {
    fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut rows = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 1 {
                    rows[i][j] = if i > 0 { rows[i - 1][j] } else { 0 } + 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            res = res.max(Self::rearrange(&mut rows[i]));
        }
        res
    }

    fn rearrange(row: &mut Vec<i32>) -> i32 {
        row.sort_unstable();
        let m = row.len();
        let mut max = 0;
        for i in 0..m {
            max = max.max(row[i] * (m - i) as i32);
        }
        max
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[0, 0, 1], [1, 1, 1], [1, 0, 1]];
    let res = 4;
    assert_eq!(Solution::largest_submatrix(matrix), res);
    let matrix = vec_vec_i32![[1, 0, 1, 0, 1]];
    let res = 3;
    assert_eq!(Solution::largest_submatrix(matrix), res);
    let matrix = vec_vec_i32![[1, 1, 0], [1, 0, 1]];
    let res = 2;
    assert_eq!(Solution::largest_submatrix(matrix), res);
    let matrix = vec_vec_i32![[0, 0], [0, 0]];
    let res = 0;
    assert_eq!(Solution::largest_submatrix(matrix), res);
}

struct Solution;

impl Solution {
    fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = row_sum.len();
        let m = col_sum.len();
        let mut i = 0;
        let mut j = 0;
        let mut res = vec![vec![0; m]; n];
        while i < n && j < m {
            let x = row_sum[i].min(col_sum[j]);
            row_sum[i] -= x;
            col_sum[j] -= x;
            res[i][j] = x;
            if row_sum[i] == 0 {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let row_sum = vec![3, 8];
    let col_sum = vec![4, 7];
    let res = vec_vec_i32![[3, 0], [1, 7]];
    assert_eq!(Solution::restore_matrix(row_sum, col_sum), res);
}

struct Solution;

impl Solution {
    fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j || i + j == n - 1 {
                    res += mat[i][j];
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = 25;
    assert_eq!(Solution::diagonal_sum(mat), res);
    let mat = vec_vec_i32![[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]];
    let res = 8;
    assert_eq!(Solution::diagonal_sum(mat), res);
    let mat = vec_vec_i32![[5]];
    let res = 5;
    assert_eq!(Solution::diagonal_sum(mat), res);
}

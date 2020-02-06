struct Solution;

impl Solution {
    fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut i = 0;
        let mut j = 0;
        let mut d = true;
        let n = matrix.len();
        if n == 0 {
            return res;
        }
        let m = matrix[0].len();
        if m == 0 {
            return res;
        }
        for _ in 0..(n * m) {
            res.push(matrix[i][j]);
            if i > 0 && j < m - 1 && d {
                i -= 1;
                j += 1;
            } else if i < n - 1 && j > 0 && !d {
                i += 1;
                j -= 1;
            } else {
                if i == 0 && j < m - 1 || i == n - 1 {
                    j += 1;
                } else if j == 0 && i < n - 1 || j == m - 1 {
                    i += 1;
                }
                d = !d;
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix: Vec<Vec<i32>> = vec_vec_i32![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let res = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
    assert_eq!(Solution::find_diagonal_order(matrix), res);
}

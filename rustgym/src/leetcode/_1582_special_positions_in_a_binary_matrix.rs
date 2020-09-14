struct Solution;

impl Solution {
    fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut rows = vec![0; n];
        let mut cols = vec![0; m];
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 && rows[i] == 1 && cols[j] == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[1, 0, 0], [0, 0, 1], [1, 0, 0]];
    let res = 1;
    assert_eq!(Solution::num_special(mat), res);
    let mat = vec_vec_i32![[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let res = 3;
    assert_eq!(Solution::num_special(mat), res);
    let mat = vec_vec_i32![[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
    let res = 2;
    assert_eq!(Solution::num_special(mat), res);
    let mat = vec_vec_i32![
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 0, 1, 1]
    ];
    let res = 3;
    assert_eq!(Solution::num_special(mat), res);
}

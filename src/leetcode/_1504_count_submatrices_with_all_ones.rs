struct Solution;

impl Solution {
    fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                let mut limit = m;
                for r in i..n {
                    for c in j..m.min(limit) {
                        if mat[r][c] == 1 {
                            res += 1;
                        } else {
                            limit = c;
                            break;
                        }
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[1, 0, 1], [1, 1, 0], [1, 1, 0]];
    let res = 13;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec_vec_i32![[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]];
    let res = 24;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec_vec_i32![[1, 1, 1, 1, 1, 1]];
    let res = 21;
    assert_eq!(Solution::num_submat(mat), res);
    let mat = vec_vec_i32![[1, 0, 1], [0, 1, 0], [1, 0, 1]];
    let res = 5;
    assert_eq!(Solution::num_submat(mat), res);
}

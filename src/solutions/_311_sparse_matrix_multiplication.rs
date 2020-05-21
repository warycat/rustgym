struct Solution;

impl Solution {
    fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let l = a.len();
        let m = b.len();
        let n = b[0].len();
        let mut res = vec![vec![0; n]; l];
        for i in 0..l {
            for j in 0..n {
                for k in 0..m {
                    res[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec_vec_i32![[1, 0, 0], [-1, 0, 3]];
    let b = vec_vec_i32![[7, 0, 0], [0, 0, 0], [0, 0, 1]];
    let res = vec_vec_i32![[7, 0, 0], [-7, 0, 3]];
    assert_eq!(Solution::multiply(a, b), res);
}

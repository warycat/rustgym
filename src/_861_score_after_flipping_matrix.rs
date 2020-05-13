struct Solution;

impl Solution {
    fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut res = n << (m - 1);
        for j in 1..m {
            let mut x = Self::sum_col(j, &a, n);
            x = x.max(n - x);
            res += x << (m - 1 - j);
        }
        res as i32
    }
    fn sum_col(j: usize, a: &[Vec<i32>], n: usize) -> usize {
        let mut res = 0;
        for i in 0..n {
            res += if a[i][j] == a[i][0] { 1 } else { 0 };
        }
        res as usize
    }
}

#[test]
fn test() {
    let a = vec_vec_i32![[0, 0, 1, 1], [1, 0, 1, 0], [1, 1, 0, 0]];
    let res = 39;
    assert_eq!(Solution::matrix_score(a), res);
    let a = vec_vec_i32![[0, 1], [0, 1], [0, 1], [0, 0]];
    let res = 11;
    assert_eq!(Solution::matrix_score(a), res);
}

struct Solution;

impl Solution {
    fn num_enclaves(mut a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == 1 && (i == 0 || j == 0 || i + 1 == n || j + 1 == m) {
                    Self::dfs(i, j, &mut a, n, m);
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == 1 {
                    res += 1;
                }
            }
        }

        res as i32
    }

    fn dfs(i: usize, j: usize, a: &mut Vec<Vec<i32>>, n: usize, m: usize) {
        a[i][j] = 0;
        if i > 0 && a[i - 1][j] == 1 {
            Self::dfs(i - 1, j, a, n, m);
        }
        if j > 0 && a[i][j - 1] == 1 {
            Self::dfs(i, j - 1, a, n, m);
        }
        if i + 1 < n && a[i + 1][j] == 1 {
            Self::dfs(i + 1, j, a, n, m);
        }
        if j + 1 < m && a[i][j + 1] == 1 {
            Self::dfs(i, j + 1, a, n, m);
        }
    }
}
#[test]
fn test() {
    let a = vec_vec_i32![[0, 0, 0, 0], [1, 0, 1, 0], [0, 1, 1, 0], [0, 0, 0, 0]];
    let res = 3;
    assert_eq!(Solution::num_enclaves(a), res);
    let a = vec_vec_i32![[0, 1, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 0]];
    let res = 0;
    assert_eq!(Solution::num_enclaves(a), res);
}

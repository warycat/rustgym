struct Solution;

impl Solution {
    fn min_flips(mut mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut res = std::usize::MAX;
        Self::dfs(0, 0, &mut mat, &mut res, n, m);
        if res == std::usize::MAX {
            -1
        } else {
            res as i32
        }
    }

    fn dfs(start: usize, k: usize, mat: &mut Vec<Vec<i32>>, min: &mut usize, n: usize, m: usize) {
        if start == n * m {
            if Self::ones(mat, n, m) == 0 {
                *min = (*min).min(k);
            }
        } else {
            let r = start / m;
            let c = start % m;
            Self::flip(r, c, mat, n, m);
            Self::dfs(start + 1, k + 1, mat, min, n, m);
            Self::flip(r, c, mat, n, m);
            Self::dfs(start + 1, k, mat, min, n, m);
        }
    }

    fn flip(i: usize, j: usize, mat: &mut Vec<Vec<i32>>, n: usize, m: usize) {
        mat[i][j] = 1 - mat[i][j];
        if i > 0 {
            mat[i - 1][j] = 1 - mat[i - 1][j];
        }
        if j > 0 {
            mat[i][j - 1] = 1 - mat[i][j - 1];
        }
        if i + 1 < n {
            mat[i + 1][j] = 1 - mat[i + 1][j];
        }
        if j + 1 < m {
            mat[i][j + 1] = 1 - mat[i][j + 1];
        }
    }

    fn ones(mat: &[Vec<i32>], n: usize, m: usize) -> usize {
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if mat[i][j] == 1 {
                    res += 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![[0, 0], [0, 1]];
    let res = 3;
    assert_eq!(Solution::min_flips(mat), res);
    let mat = vec_vec_i32![[0]];
    let res = 0;
    assert_eq!(Solution::min_flips(mat), res);
    let mat = vec_vec_i32![[1, 1, 1], [1, 0, 1], [0, 0, 0]];
    let res = 6;
    assert_eq!(Solution::min_flips(mat), res);
    let mat = vec_vec_i32![[1, 0, 0], [1, 0, 0]];
    let res = -1;
    assert_eq!(Solution::min_flips(mat), res);
}

struct Solution;

impl Solution {
    fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                res = res.max(Self::translate(i, j, &a, &b, n));
                res = res.max(Self::translate(i, j, &b, &a, n));
            }
        }
        res
    }

    fn translate(x: usize, y: usize, a: &[Vec<i32>], b: &[Vec<i32>], n: usize) -> i32 {
        let mut res = 0;
        for i in 0..n {
            for j in 0..n {
                if i + x < n && j + y < n {
                    res += a[i + x][j + y] * b[i][j];
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec_vec_i32![[1, 1, 0], [0, 1, 0], [0, 1, 0]];
    let b = vec_vec_i32![[0, 0, 0], [0, 1, 1], [0, 0, 1]];
    let res = 3;
    assert_eq!(Solution::largest_overlap(a, b), res);
}

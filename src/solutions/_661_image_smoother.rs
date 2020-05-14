struct Solution;

impl Solution {
    fn smooth(m: &[Vec<i32>], r: usize, c: usize, h: usize, w: usize) -> i32 {
        let mut sum = 0;
        let mut n = 0;
        if r > 0 && c > 0 {
            sum += m[r - 1][c - 1];
            n += 1;
        }
        if r > 0 {
            sum += m[r - 1][c];
            n += 1;
        }
        if r > 0 && c < w - 1 {
            sum += m[r - 1][c + 1];
            n += 1;
        }
        if c > 0 {
            sum += m[r][c - 1];
            n += 1;
        }
        sum += m[r][c];
        n += 1;
        if c < w - 1 {
            sum += m[r][c + 1];
            n += 1;
        }
        if r < h - 1 && c > 0 {
            sum += m[r + 1][c - 1];
            n += 1;
        }
        if r < h - 1 {
            sum += m[r + 1][c];
            n += 1;
        }
        if r < h - 1 && c < w - 1 {
            sum += m[r + 1][c + 1];
            n += 1;
        }
        sum / n
    }
    fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let h = m.len();
        let w = m[0].len();
        let mut res = vec![vec![0; w]; h];
        for i in 0..h {
            for j in 0..w {
                res[i][j] = Self::smooth(&m, i, j, h, w);
            }
        }
        res
    }
}

#[test]
fn test() {
    let m: Vec<Vec<i32>> = vec_vec_i32![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
    let o: Vec<Vec<i32>> = vec_vec_i32![[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    assert_eq!(Solution::image_smoother(m), o);
}

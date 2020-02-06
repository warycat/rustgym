struct Solution;

impl Solution {
    fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut a = vec![vec![0; m]; n];
        let mut res = 0;
        for p in indices {
            let r = p[0] as usize;
            let c = p[1] as usize;
            for j in 0..m {
                a[r][j] += 1;
                if a[r][j] % 2 == 1 {
                    res += 1;
                } else {
                    res -= 1;
                }
            }
            for i in 0..n {
                a[i][c] += 1;
                if a[i][c] % 2 == 1 {
                    res += 1;
                } else {
                    res -= 1;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let m = 3;
    let indices: Vec<Vec<i32>> = vec_vec_i32![[0, 1], [1, 1]];
    assert_eq!(Solution::odd_cells(n, m, indices), 6);
    let n = 2;
    let m = 2;
    let indices: Vec<Vec<i32>> = vec_vec_i32![[1, 1], [0, 0]];
    assert_eq!(Solution::odd_cells(n, m, indices), 0);
}

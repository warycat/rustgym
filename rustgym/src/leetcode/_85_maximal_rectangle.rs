struct Solution;

impl Solution {
    fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix.len();
        if n == 0 {
            return 0;
        }
        let m = matrix[0].len();
        let mut left = vec![0; m];
        let mut right = vec![m; m];
        let mut height = vec![0; m];
        let mut res = 0;
        for i in 0..n {
            let mut l = 0;
            let mut r = m;
            for j in 0..m {
                if matrix[i][j] == '1' {
                    height[j] += 1;
                } else {
                    height[j] = 0;
                }
            }
            for j in 0..m {
                if matrix[i][j] == '1' {
                    left[j] = left[j].max(l);
                } else {
                    left[j] = 0;
                    l = j + 1;
                }
            }
            for j in (0..m).rev() {
                if matrix[i][j] == '1' {
                    right[j] = right[j].min(r);
                } else {
                    right[j] = m;
                    r = j;
                }
            }
            for j in 0..m {
                res = res.max((right[j] - left[j]) * height[j]);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let matrix = vec_vec_char![
        ['1', '0', '1', '0', '0'],
        ['1', '0', '1', '1', '1'],
        ['1', '1', '1', '1', '1'],
        ['1', '0', '0', '1', '0']
    ];
    let res = 6;
    assert_eq!(Solution::maximal_rectangle(matrix), res);
}

struct Solution;

impl Solution {
    fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut prefix = vec![vec![0; m + 1]; n + 1];
        let mut k = 1;
        for i in 0..n {
            for j in 0..m {
                prefix[i][j] = mat[i][j];
                if i > 0 {
                    prefix[i][j] += prefix[i - 1][j];
                }
                if j > 0 {
                    prefix[i][j] += prefix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    prefix[i][j] -= prefix[i - 1][j - 1];
                }
                if i >= k - 1 && j >= k - 1 {
                    let mut sum = prefix[i][j];
                    if i >= k {
                        sum -= prefix[i - k][j];
                    }
                    if j >= k {
                        sum -= prefix[i][j - k];
                    }
                    if i >= k && j >= k {
                        sum += prefix[i - k][j - k];
                    }
                    if sum <= threshold {
                        k += 1;
                    }
                }
            }
        }
        (k - 1) as i32
    }
}

#[test]
fn test() {
    let mat = vec_vec_i32![
        [1, 1, 3, 2, 4, 3, 2],
        [1, 1, 3, 2, 4, 3, 2],
        [1, 1, 3, 2, 4, 3, 2]
    ];
    let threshold = 4;
    let res = 2;
    assert_eq!(Solution::max_side_length(mat, threshold), res);
    let mat = vec_vec_i32![
        [2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2]
    ];
    let threshold = 1;
    let res = 0;
    assert_eq!(Solution::max_side_length(mat, threshold), res);
    let mat = vec_vec_i32![[1, 1, 1, 1], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]];
    let threshold = 6;
    let res = 3;
    assert_eq!(Solution::max_side_length(mat, threshold), res);
    let mat = vec_vec_i32![
        [18, 70],
        [61, 1],
        [25, 85],
        [14, 40],
        [11, 96],
        [97, 96],
        [63, 45]
    ];
    let threshold = 40184;
    let res = 2;
    assert_eq!(Solution::max_side_length(mat, threshold), res);
}

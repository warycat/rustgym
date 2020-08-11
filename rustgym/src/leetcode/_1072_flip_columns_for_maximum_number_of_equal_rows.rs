struct Solution;

impl Solution {
    fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut res = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                let count: usize = matrix[i]
                    .iter()
                    .zip(matrix[j].iter())
                    .map(|(x, y)| if x == y { 1 } else { 0 })
                    .sum();
                if count == 0 || count == m {
                    res[i] += 1;
                    res[j] += 1;
                }
            }
        }
        *res.iter().max().unwrap() as i32
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[0, 1], [1, 1]];
    let res = 1;
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), res);
    let matrix = vec_vec_i32![[0, 1], [1, 0]];
    let res = 2;
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), res);
    let matrix = vec_vec_i32![[0, 0, 0], [0, 0, 1], [1, 1, 0]];
    let res = 2;
    assert_eq!(Solution::max_equal_rows_after_flips(matrix), res);
}

struct Solution;

impl Solution {
    fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut mins: Vec<i32> = vec![std::i32::MAX; n];
        let mut maxs: Vec<i32> = vec![std::i32::MIN; m];
        let mut res = vec![];
        for i in 0..n {
            for j in 0..m {
                mins[i] = mins[i].min(matrix[i][j]);
                maxs[j] = maxs[j].max(matrix[i][j]);
            }
        }
        for i in 0..n {
            for j in 0..m {
                if mins[i] == matrix[i][j] && maxs[j] == matrix[i][j] {
                    res.push(matrix[i][j]);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[3, 7, 8], [9, 11, 13], [15, 16, 17]];
    let res = vec![15];
    assert_eq!(Solution::lucky_numbers(matrix), res);
    let matrix = vec_vec_i32![[1, 10, 4, 2], [9, 3, 8, 7], [15, 16, 17, 12]];
    let res = vec![12];
    assert_eq!(Solution::lucky_numbers(matrix), res);
    let matrix = vec_vec_i32![[7, 8], [1, 2]];
    let res = vec![7];
    assert_eq!(Solution::lucky_numbers(matrix), res);
}

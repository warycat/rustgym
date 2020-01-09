struct Solution;

impl Solution {
    fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
    }
}

#[test]
fn test() {
    let mut matrix: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    let res: Vec<Vec<i32>> = [[7, 4, 1], [8, 5, 2], [9, 6, 3]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, res);
}

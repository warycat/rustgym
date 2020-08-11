struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    queue.push_back((i, j, 0));
                }
            }
        }
        while let Some((i, j, d)) = queue.pop_front() {
            if visited[i][j] {
                continue;
            }
            visited[i][j] = true;
            matrix[i][j] = d;
            if i > 0 {
                queue.push_back((i - 1, j, d + 1));
            }
            if j > 0 {
                queue.push_back((i, j - 1, d + 1));
            }
            if i + 1 < n {
                queue.push_back((i + 1, j, d + 1));
            }
            if j + 1 < m {
                queue.push_back((i, j + 1, d + 1));
            }
        }
        matrix
    }
}

#[test]
fn test() {
    let matrix = vec_vec_i32![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
    let res = vec_vec_i32![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
    assert_eq!(Solution::update_matrix(matrix), res);
    let matrix = vec_vec_i32![[0, 0, 0], [0, 1, 0], [1, 1, 1]];
    let res = vec_vec_i32![[0, 0, 0], [0, 1, 0], [1, 2, 1]];
    assert_eq!(Solution::update_matrix(matrix), res);
    let matrix = vec_vec_i32![[0], [0], [0], [0], [0]];
    let res = vec_vec_i32![[0], [0], [0], [0], [0]];
    assert_eq!(Solution::update_matrix(matrix), res);
}

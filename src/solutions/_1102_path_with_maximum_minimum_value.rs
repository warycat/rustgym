struct Solution;
use std::collections::BinaryHeap;

impl Solution {
    fn maximum_minimum_path(a: Vec<Vec<i32>>) -> i32 {
        let n = a.len();
        let m = a[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        let mut queue: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        visited[0][0] = true;
        queue.push((a[0][0], 0, 0));
        let mut res = std::i32::MAX;
        while let Some((x, i, j)) = queue.pop() {
            res = res.min(x);
            if i == n - 1 && j == m - 1 {
                break;
            }
            if i > 0 && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                queue.push((a[i - 1][j], i - 1, j));
            }
            if j > 0 && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                queue.push((a[i][j - 1], i, j - 1));
            }
            if i + 1 < n && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                queue.push((a[i + 1][j], i + 1, j));
            }
            if j + 1 < m && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                queue.push((a[i][j + 1], i, j + 1));
            }
        }
        res
    }
}

#[test]
fn test() {
    let a = vec_vec_i32![[5, 4, 5], [1, 2, 6], [7, 4, 6]];
    let res = 4;
    assert_eq!(Solution::maximum_minimum_path(a), res);
    let a = vec_vec_i32![[2, 2, 1, 2, 2, 2], [1, 2, 2, 2, 1, 2]];
    let res = 2;
    assert_eq!(Solution::maximum_minimum_path(a), res);
    let a = vec_vec_i32![
        [3, 4, 6, 3, 4],
        [0, 2, 1, 1, 7],
        [8, 8, 3, 2, 7],
        [3, 2, 4, 9, 8],
        [4, 1, 2, 0, 0],
        [4, 6, 5, 4, 3]
    ];
    let res = 3;
    assert_eq!(Solution::maximum_minimum_path(a), res);
}

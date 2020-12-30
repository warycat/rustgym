struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

impl Solution {
    fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let n = forest.len();
        let m = forest[0].len();
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        for i in 0..n {
            for j in 0..m {
                if forest[i][j] > 1 {
                    queue.push((Reverse(forest[i][j]), i, j));
                }
            }
        }
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;
        while let Some((_, r, c)) = queue.pop() {
            let d = Self::bfs(i, j, r, c, &forest, n, m);
            if d < 0 {
                return -1;
            }
            i = r;
            j = c;
            res += d;
        }
        res
    }

    fn bfs(i: usize, j: usize, r: usize, c: usize, forest: &[Vec<i32>], n: usize, m: usize) -> i32 {
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        queue.push_back((i, j, 0));
        visited[i][j] = true;
        while let Some((i, j, d)) = queue.pop_front() {
            if r == i && c == j {
                return d;
            }
            if i > 0 && forest[i - 1][j] > 0 && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                queue.push_back((i - 1, j, d + 1));
            }
            if j > 0 && forest[i][j - 1] > 0 && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                queue.push_back((i, j - 1, d + 1));
            }
            if i + 1 < n && forest[i + 1][j] > 0 && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                queue.push_back((i + 1, j, d + 1));
            }
            if j + 1 < m && forest[i][j + 1] > 0 && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                queue.push_back((i, j + 1, d + 1));
            }
        }
        -1
    }
}

#[test]
fn test() {
    let forest = vec_vec_i32![[1, 2, 3], [0, 0, 4], [7, 6, 5]];
    let res = 6;
    assert_eq!(Solution::cut_off_tree(forest), res);
    let forest = vec_vec_i32![[1, 2, 3], [0, 0, 0], [7, 6, 5]];
    let res = -1;
    assert_eq!(Solution::cut_off_tree(forest), res);
    let forest = vec_vec_i32![[2, 3, 4], [0, 0, 5], [8, 7, 6]];
    let res = 6;
    assert_eq!(Solution::cut_off_tree(forest), res);
}

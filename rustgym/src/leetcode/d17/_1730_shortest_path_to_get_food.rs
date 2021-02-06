struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn get_food(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];
        'outer: for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '*' {
                    queue.push_back((i, j, 0));
                    visited[i][j] = true;
                    break 'outer;
                }
            }
        }
        while let Some((i, j, d)) = queue.pop_front() {
            for (di, dj) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if grid[i][j] == '#' {
                    return d;
                }
                let i = i as i32 + di;
                let j = j as i32 + dj;
                if 0 <= i && i < n as i32 && 0 <= j && j < m as i32 {
                    let i = i as usize;
                    let j = j as usize;
                    if !visited[i][j] && grid[i][j] != 'X' {
                        queue.push_back((i, j, d + 1));
                        visited[i][j] = true;
                    }
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let grid = vec_vec_char![
        ['X', 'X', 'X', 'X', 'X', 'X'],
        ['X', '*', 'O', 'O', 'O', 'X'],
        ['X', 'O', 'O', '#', 'O', 'X'],
        ['X', 'X', 'X', 'X', 'X', 'X']
    ];
    let res = 3;
    assert_eq!(Solution::get_food(grid), res);
    let grid = vec_vec_char![
        ['X', 'X', 'X', 'X', 'X'],
        ['X', '*', 'X', 'O', 'X'],
        ['X', 'O', 'X', '#', 'X'],
        ['X', 'X', 'X', 'X', 'X']
    ];
    let res = -1;
    assert_eq!(Solution::get_food(grid), res);
    let grid = vec_vec_char![
        ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
        ['X', '*', 'O', 'X', 'O', '#', 'O', 'X'],
        ['X', 'O', 'O', 'X', 'O', 'O', 'X', 'X'],
        ['X', 'O', 'O', 'O', 'O', '#', 'O', 'X'],
        ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X']
    ];
    let res = 6;
    assert_eq!(Solution::get_food(grid), res);
    let grid = vec_vec_char![['O', '*'], ['#', 'O']];
    let res = 2;
    assert_eq!(Solution::get_food(grid), res);
    let grid = vec_vec_char![['X', '*'], ['#', 'X']];
    let res = -1;
    assert_eq!(Solution::get_food(grid), res);
}

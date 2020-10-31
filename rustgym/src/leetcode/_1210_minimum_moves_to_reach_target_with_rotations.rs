struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

type State = (usize, usize, bool);

impl Solution {
    fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited: HashSet<State> = HashSet::new();
        let start = (0, 0, false);
        let finish = (n - 1, n - 2, false);
        visited.insert(start);
        let mut queue: VecDeque<State> = VecDeque::new();
        queue.push_back(start);

        let mut res = 0;
        while !queue.is_empty() {
            let m = queue.len();
            for _ in 0..m {
                let state = queue.pop_front().unwrap();
                if state == finish {
                    return res;
                }
                let i = state.0;
                let j = state.1;
                let d = state.2;
                let right = (i, j + 1, d);
                let down = (i + 1, j, d);
                let rotate = (i, j, !d);
                if d {
                    if j + 1 < n && grid[i][j + 1] == 0 && grid[i + 1][j + 1] == 0 {
                        if visited.insert(right) {
                            queue.push_back(right);
                        }
                        if visited.insert(rotate) {
                            queue.push_back(rotate);
                        }
                    }
                    if i + 2 < n && grid[i + 2][j] == 0 && visited.insert(down) {
                        queue.push_back(down);
                    }
                } else {
                    if i + 1 < n && grid[i + 1][j] == 0 && grid[i + 1][j + 1] == 0 {
                        if visited.insert(down) {
                            queue.push_back(down);
                        }
                        if visited.insert(rotate) {
                            queue.push_back(rotate);
                        }
                    }
                    if j + 2 < n && grid[i][j + 2] == 0 && visited.insert(right) {
                        queue.push_back(right);
                    }
                }
            }
            res += 1;
        }
        -1
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![
        [0, 0, 0, 0, 0, 1],
        [1, 1, 0, 0, 1, 0],
        [0, 0, 0, 0, 1, 1],
        [0, 0, 1, 0, 1, 0],
        [0, 1, 1, 0, 0, 0],
        [0, 1, 1, 0, 0, 0]
    ];
    let res = 11;
    assert_eq!(Solution::minimum_moves(grid), res);
    let grid = vec_vec_i32![
        [0, 0, 1, 1, 1, 1],
        [0, 0, 0, 0, 1, 1],
        [1, 1, 0, 0, 0, 1],
        [1, 1, 1, 0, 0, 1],
        [1, 1, 1, 0, 0, 1],
        [1, 1, 1, 0, 0, 0]
    ];
    let res = 9;
    assert_eq!(Solution::minimum_moves(grid), res);
}

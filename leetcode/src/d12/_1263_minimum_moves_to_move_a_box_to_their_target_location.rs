struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type State = (i32, i32, i32, i32);

impl Solution {
    fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut floors: HashSet<(i32, i32)> = HashSet::new();
        let mut state = (0, 0, 0, 0);
        let mut t = (0, 0);
        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    'S' => {
                        state.0 = i as i32;
                        state.1 = j as i32;
                        floors.insert((state.0, state.1));
                    }
                    'B' => {
                        state.2 = i as i32;
                        state.3 = j as i32;
                        floors.insert((state.2, state.3));
                    }
                    'T' => {
                        t = (i as i32, j as i32);
                        floors.insert(t);
                    }
                    '.' => {
                        floors.insert((i as i32, j as i32));
                    }
                    _ => {}
                }
            }
        }
        let mut queue: BinaryHeap<(Reverse<i32>, State)> = BinaryHeap::new();
        queue.push((Reverse(0), state));
        let mut visited: HashSet<State> = HashSet::new();
        visited.insert(state);
        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        while let Some((Reverse(step), state)) = queue.pop() {
            if (state.2, state.3) == t {
                return step;
            }
            for (di, dj) in &dirs {
                let pi = state.0 + di;
                let pj = state.1 + dj;
                let bi = state.2;
                let bj = state.3;
                if (state.2, state.3) == (pi, pj) {
                    let next_state = (pi, pj, bi + di, bj + dj);
                    if floors.contains(&(bi, bj)) && visited.insert(next_state) {
                        queue.push((Reverse(step + 1), next_state));
                    }
                } else {
                    let next_state = (pi, pj, bi, bj);
                    if floors.contains(&(pi, pj)) && visited.insert(next_state) {
                        queue.push((Reverse(step), next_state));
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
        ['#', '#', '#', '#', '#', '#'],
        ['#', 'T', '#', '#', '#', '#'],
        ['#', '.', '.', 'B', '.', '#'],
        ['#', '.', '#', '#', '.', '#'],
        ['#', '.', '.', '.', 'S', '#'],
        ['#', '#', '#', '#', '#', '#']
    ];
    let res = 3;
    assert_eq!(Solution::min_push_box(grid), res);
    let grid = vec_vec_char![
        ['#', '#', '#', '#', '#', '#'],
        ['#', 'T', '#', '#', '#', '#'],
        ['#', '.', '.', 'B', '.', '#'],
        ['#', '#', '#', '#', '.', '#'],
        ['#', '.', '.', '.', 'S', '#'],
        ['#', '#', '#', '#', '#', '#']
    ];
    let res = -1;
    assert_eq!(Solution::min_push_box(grid), res);
    let grid = vec_vec_char![
        ['#', '#', '#', '#', '#', '#'],
        ['#', 'T', '.', '.', '#', '#'],
        ['#', '.', '#', 'B', '.', '#'],
        ['#', '.', '.', '.', '.', '#'],
        ['#', '.', '.', '.', 'S', '#'],
        ['#', '#', '#', '#', '#', '#']
    ];
    let res = 5;
    assert_eq!(Solution::min_push_box(grid), res);
    let grid = vec_vec_char![
        ['#', '#', '#', '#', '#', '#', '#'],
        ['#', 'S', '#', '.', 'B', 'T', '#'],
        ['#', '#', '#', '#', '#', '#', '#']
    ];
    let res = -1;
    assert_eq!(Solution::min_push_box(grid), res);
    let grid = vec_vec_char![
        ['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '#', '.', '#', 'B', '#', '.', '#', '.', '.'],
        ['.', '#', '.', '.', '.', '.', '.', '.', 'T', '.'],
        ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
        ['.', '.', '.', '#', '.', '.', '#', '#', '.', '.'],
        ['.', '.', '.', '.', '#', '.', '.', '#', '.', '.'],
        ['.', '#', '.', 'S', '.', '.', '.', '.', '.', '.'],
        ['#', '.', '.', '#', '.', '.', '.', '.', '.', '#']
    ];
    let res = 5;
    assert_eq!(Solution::min_push_box(grid), res);
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
        let start_r = ball[0] as usize;
        let start_c = ball[1] as usize;
        let end_r = hole[0] as usize;
        let end_c = hole[1] as usize;
        let n = maze.len();
        let m = maze[0].len();
        let mut states = vec![vec![(std::usize::MAX, "".to_string()); m]; n];
        let mut queue: BinaryHeap<(Reverse<usize>, Reverse<String>, usize, usize)> =
            BinaryHeap::new();
        queue.push((Reverse(0), Reverse("".to_string()), start_r, start_c));
        while let Some((Reverse(dist), Reverse(path), r, c)) = queue.pop() {
            if r == end_r && c == end_c {
                return path;
            }
            let mut i = r;
            let j = c;
            let mut d = 0;
            let mut p = path.to_string();
            p.push('u');
            while i > 0 && maze[i - 1][j] == 0 {
                i -= 1;
                d += 1;
                if i == end_r && j == end_c {
                    break;
                }
            }
            if d > 0 && (d, p.to_string()) < states[i][j] {
                states[i][j] = (d, p.to_string());
                queue.push((Reverse(dist + d), Reverse(p), i, j));
            }

            let i = r;
            let mut j = c;
            let mut d = 0;
            let mut p = path.to_string();
            p.push('l');
            while j > 0 && maze[i][j - 1] == 0 {
                j -= 1;
                d += 1;
                if i == end_r && j == end_c {
                    break;
                }
            }
            if d > 0 && (d, p.to_string()) < states[i][j] {
                states[i][j] = (d, p.to_string());
                queue.push((Reverse(dist + d), Reverse(p), i, j));
            }

            let mut i = r;
            let j = c;
            let mut d = 0;
            let mut p = path.to_string();
            p.push('d');
            while i + 1 < n && maze[i + 1][j] == 0 {
                i += 1;
                d += 1;
                if i == end_r && j == end_c {
                    break;
                }
            }
            if d > 0 && (d, p.to_string()) < states[i][j] {
                states[i][j] = (d, p.to_string());
                queue.push((Reverse(dist + d), Reverse(p), i, j));
            }

            let i = r;
            let mut j = c;
            let mut d = 0;
            let mut p = path.to_string();
            p.push('r');
            while j + 1 < m && maze[i][j + 1] == 0 {
                j += 1;
                d += 1;
                if i == end_r && j == end_c {
                    break;
                }
            }
            if d > 0 && (d, p.to_string()) < states[i][j] {
                states[i][j] = (d, p.to_string());
                queue.push((Reverse(dist + d), Reverse(p), i, j));
            }
        }
        "impossible".to_string()
    }
}

#[test]
fn test() {
    let maze = vec_vec_i32![
        [0, 0, 0, 0, 0],
        [1, 1, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1],
        [0, 1, 0, 0, 0]
    ];
    let ball = vec![4, 3];
    let hole = vec![0, 1];
    let res = "lul".to_string();
    assert_eq!(Solution::find_shortest_way(maze, ball, hole), res);
    let maze = vec_vec_i32![
        [0, 0, 0, 0, 0],
        [1, 1, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 0, 1],
        [0, 1, 0, 0, 0]
    ];
    let ball = vec![4, 3];
    let hole = vec![3, 0];
    let res = "impossible".to_string();
    assert_eq!(Solution::find_shortest_way(maze, ball, hole), res);
}

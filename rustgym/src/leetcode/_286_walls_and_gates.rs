struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let n = rooms.len();
        if n == 0 {
            return;
        }
        let m = rooms[0].len();
        if m == 0 {
            return;
        }
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                if rooms[i][j] == 0 {
                    queue.push_back((i, j, 0));
                }
            }
        }
        while let Some((i, j, dist)) = queue.pop_front() {
            let dist = dist + 1;
            if i > 0 && rooms[i - 1][j] > 0 && dist < rooms[i - 1][j] {
                rooms[i - 1][j] = dist;
                queue.push_back((i - 1, j, dist));
            }
            if j > 0 && rooms[i][j - 1] > 0 && dist < rooms[i][j - 1] {
                rooms[i][j - 1] = dist;
                queue.push_back((i, j - 1, dist));
            }
            if i + 1 < n && rooms[i + 1][j] > 0 && dist < rooms[i + 1][j] {
                rooms[i + 1][j] = dist;
                queue.push_back((i + 1, j, dist));
            }
            if j + 1 < m && rooms[i][j + 1] > 0 && dist < rooms[i][j + 1] {
                rooms[i][j + 1] = dist;
                queue.push_back((i, j + 1, dist));
            }
        }
    }
}

#[test]
fn test() {
    let inf = std::i32::MAX;
    let mut rooms = vec_vec_i32![
        [inf, -1, 0, inf],
        [inf, inf, inf, -1],
        [inf, -1, inf, -1],
        [0, -1, inf, inf]
    ];
    let res = vec_vec_i32![[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, res);
}

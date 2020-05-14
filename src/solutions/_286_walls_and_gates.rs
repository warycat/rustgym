struct Solution;

use std::collections::VecDeque;

struct Point {
    i: usize,
    j: usize,
    v: i32,
}

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
        let mut queue: VecDeque<Point> = VecDeque::new();
        for i in 0..n {
            for j in 0..m {
                let v = rooms[i][j];
                if v == 0 {
                    queue.push_back(Point { i, j, v })
                }
            }
        }
        let di = vec![0, 0, -1, 1];
        let dj = vec![-1, 1, 0, 0];
        while let Some(p) = queue.pop_front() {
            let i = p.i as i32;
            let j = p.j as i32;
            let v = p.v + 1;
            for k in 0..4 {
                let i = i + di[k];
                let j = j + dj[k];
                if i < 0 || i >= n as i32 {
                    continue;
                }
                if j < 0 || j >= m as i32 {
                    continue;
                }
                let i = i as usize;
                let j = j as usize;
                if rooms[i][j] == -1 {
                    continue;
                }
                if rooms[i][j] > v {
                    rooms[i][j] = v;
                    queue.push_back(Point { i, j, v });
                }
            }
        }
    }
}

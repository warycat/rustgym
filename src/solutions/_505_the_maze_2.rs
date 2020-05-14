struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        let n = maze.len();
        let m = maze[0].len();
        let r = start[0] as usize;
        let c = start[1] as usize;
        let x = destination[0] as usize;
        let y = destination[1] as usize;
        let mut dist: Vec<Vec<i32>> = vec![vec![-1; m]; n];
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        queue.push_back((r, c, 0));
        while let Some((i, j, d)) = queue.pop_front() {
            if dist[i][j] != -1 && d >= dist[i][j] {
                continue;
            }
            dist[i][j] = d;
            let mut step = 0;
            let mut di = i;
            while di > 0 && maze[di - 1][j] == 0 {
                step += 1;
                di -= 1;
            }
            queue.push_back((di, j, d + step));

            let mut step = 0;
            let mut dj = j;
            while dj > 0 && maze[i][dj - 1] == 0 {
                step += 1;
                dj -= 1;
            }
            queue.push_back((i, dj, d + step));

            let mut step = 0;
            let mut di = i;
            while di + 1 < n && maze[di + 1][j] == 0 {
                step += 1;
                di += 1;
            }
            queue.push_back((di, j, d + step));

            let mut step = 0;
            let mut dj = j;
            while dj + 1 < m && maze[i][dj + 1] == 0 {
                step += 1;
                dj += 1;
            }
            queue.push_back((i, dj, d + step));
        }
        dist[x][y]
    }
}

#[test]
fn test() {
    let maze: Vec<Vec<i32>> = vec_vec_i32![
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 1, 1],
        [0, 0, 0, 0, 0]
    ];
    let start = vec![0, 4];
    let destination = vec![4, 4];
    let res = 12;
    assert_eq!(Solution::shortest_distance(maze, start, destination), res);
    let maze: Vec<Vec<i32>> = vec_vec_i32![
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 1, 1],
        [0, 0, 0, 0, 0]
    ];
    let start = vec![0, 4];
    let destination = vec![3, 2];
    let res = -1;
    assert_eq!(Solution::shortest_distance(maze, start, destination), res);
}

struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize - 1;
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for time in times {
            let u = time[0] as usize - 1;
            let v = time[1] as usize - 1;
            let t = time[2];
            graph[u].push((v, t));
        }
        let mut visited = vec![std::i32::MAX; n];
        let mut queue = VecDeque::new();
        visited[k] = 0;
        queue.push_back(k);
        while let Some(u) = queue.pop_front() {
            for &(v, t) in &graph[u] {
                if t + visited[u] < visited[v] {
                    visited[v] = t + visited[u];
                    queue.push_back(v);
                }
            }
        }
        let max = visited.into_iter().max().unwrap();
        if max == std::i32::MAX {
            -1
        } else {
            max
        }
    }
}

#[test]
fn test() {
    let times = vec_vec_i32![[2, 1, 1], [2, 3, 1], [3, 4, 1]];
    let n = 4;
    let k = 2;
    let res = 2;
    assert_eq!(Solution::network_delay_time(times, n, k), res);
}

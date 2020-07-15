struct Solution;
use std::collections::VecDeque;

impl Solution {
    fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        let mut degree = vec![0; n];
        for edge in relations {
            let x = edge[0] as usize - 1;
            let y = edge[1] as usize - 1;
            adj[x].push(y);
            degree[y] += 1;
        }
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            if degree[i] == 0 {
                visited[i] = true;
                queue.push_back(i);
            }
        }
        let mut res = 0;
        while !queue.is_empty() {
            let n = queue.len();
            res += 1;
            for _ in 0..n {
                let u = queue.pop_front().unwrap();
                for &v in adj[u].iter() {
                    degree[v] -= 1;
                    if !visited[v] && degree[v] == 0 {
                        visited[v] = true;
                        queue.push_back(v);
                    }
                }
            }
        }
        if visited.into_iter().all(|x| x) {
            res
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let relations = vec_vec_i32![[1, 3], [2, 3]];
    let res = 2;
    assert_eq!(Solution::minimum_semesters(n, relations), res);
    let n = 3;
    let relations = vec_vec_i32![[1, 2], [2, 3], [3, 1]];
    let res = -1;
    assert_eq!(Solution::minimum_semesters(n, relations), res);
}

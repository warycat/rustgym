struct Solution;
use std::collections::VecDeque;
impl Solution {
    fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut visited: Vec<bool> = vec![false; n];
        let mut degree: Vec<usize> = vec![0; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        let mut leaves: VecDeque<usize> = VecDeque::new();
        for i in 0..n {
            if graph[i].len() == 1 {
                leaves.push_back(i);
            }
        }
        let mut m = n;
        while m > 2 {
            m -= leaves.len();
            for _ in 0..leaves.len() {
                let u = leaves.pop_front().unwrap();
                visited[u] = true;
                for &v in &graph[u] {
                    if !visited[v] {
                        degree[v] -= 1;
                        if degree[v] == 1 {
                            leaves.push_back(v);
                        }
                    }
                }
            }
        }
        leaves.into_iter().map(|x| x as i32).collect()
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec_vec_i32![[1, 0], [1, 2], [1, 3]];
    let mut res = vec![1];
    let mut ans = Solution::find_min_height_trees(n, edges);
    ans.sort_unstable();
    res.sort_unstable();
    assert_eq!(ans, res);
    let n = 6;
    let edges = vec_vec_i32![[0, 3], [1, 3], [2, 3], [4, 3], [5, 4]];
    let mut res = vec![3, 4];
    let mut ans = Solution::find_min_height_trees(n, edges);
    ans.sort_unstable();
    res.sort_unstable();
    assert_eq!(ans, res);
    let n = 3;
    let edges = vec_vec_i32![[0, 1], [0, 2]];
    let mut res = vec![0];
    let mut ans = Solution::find_min_height_trees(n, edges);
    ans.sort_unstable();
    res.sort_unstable();
    assert_eq!(ans, res);
}

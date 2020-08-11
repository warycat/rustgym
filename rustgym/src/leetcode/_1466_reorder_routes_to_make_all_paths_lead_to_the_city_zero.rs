struct Solution;

impl Solution {
    fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<(usize, bool)>> = vec![vec![]; n];
        for connection in connections {
            let u = connection[0] as usize;
            let v = connection[1] as usize;
            graph[u].push((v, true));
            graph[v].push((u, false));
        }
        let mut res = 0;
        let mut visited = vec![false; n];
        visited[0] = true;
        Self::dfs(0, &mut visited, &mut res, &graph);
        res
    }

    fn dfs(u: usize, visited: &mut [bool], changed: &mut i32, graph: &[Vec<(usize, bool)>]) {
        for &(v, d) in &graph[u] {
            if !visited[v] {
                if d {
                    *changed += 1;
                }
                visited[v] = true;
                Self::dfs(v, visited, changed, graph);
            }
        }
    }
}

#[test]
fn test() {
    let n = 6;
    let connections = vec_vec_i32![[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]];
    let res = 3;
    assert_eq!(Solution::min_reorder(n, connections), res);
    let n = 5;
    let connections = vec_vec_i32![[1, 0], [1, 2], [3, 2], [3, 4]];
    let res = 2;
    assert_eq!(Solution::min_reorder(n, connections), res);
    let n = 3;
    let connections = vec_vec_i32![[1, 0], [2, 0]];
    let res = 0;
    assert_eq!(Solution::min_reorder(n, connections), res);
}

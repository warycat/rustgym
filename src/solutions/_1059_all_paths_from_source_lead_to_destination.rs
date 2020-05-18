struct Solution;
impl Solution {
    fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let n = n as usize;
        let s = source as usize;
        let d = destination as usize;
        let mut visited: Vec<bool> = vec![false; n];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
        }
        if !graph[d].is_empty() {
            return false;
        }
        Self::dfs(s, &mut visited, &graph, d)
    }

    fn dfs(u: usize, visited: &mut [bool], graph: &[Vec<usize>], d: usize) -> bool {
        if u == d {
            true
        } else {
            if visited[u] {
                false
            } else {
                visited[u] = true;
                let mut count = 0;
                for &v in &graph[u] {
                    if !Self::dfs(v, visited, graph, d) {
                        return false;
                    } else {
                        count += 1;
                    }
                }
                visited[u] = false;
                count != 0
            }
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let edges = vec_vec_i32![[0, 1], [0, 2]];
    let source = 0;
    let destination = 2;
    let res = false;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
    let n = 4;
    let edges = vec_vec_i32![[0, 1], [0, 3], [1, 2], [2, 1]];
    let source = 0;
    let destination = 3;
    let res = false;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
    let n = 4;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 3], [2, 3]];
    let source = 0;
    let destination = 3;
    let res = true;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
    let n = 3;
    let edges = vec_vec_i32![[0, 1], [1, 1], [1, 2]];
    let source = 0;
    let destination = 2;
    let res = false;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
    let n = 2;
    let edges = vec_vec_i32![[0, 1], [1, 1]];
    let source = 0;
    let destination = 1;
    let res = false;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
    let n = 4;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 3], [2, 3], [1, 2]];
    let source = 0;
    let destination = 3;
    let res = true;
    assert_eq!(
        Solution::leads_to_destination(n, edges, source, destination),
        res
    );
}

struct Solution;

impl Solution {
    fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut visited = vec![false; n];
        let mut counts: Vec<usize> = vec![0; 26];
        let mut res = vec![0; n];
        let labels: Vec<u8> = labels.bytes().collect();
        Self::dfs(0, &mut visited, &mut counts, &mut res, &adj, &labels);
        res
    }

    fn dfs(
        u: usize,
        visited: &mut Vec<bool>,
        counts: &mut Vec<usize>,
        sizes: &mut Vec<i32>,
        adj: &[Vec<usize>],
        labels: &[u8],
    ) {
        visited[u] = true;
        let i = (labels[u] - b'a') as usize;
        let last_count = counts[i];
        counts[i] += 1;
        for &v in adj[u].iter() {
            if !visited[v] {
                Self::dfs(v, visited, counts, sizes, adj, labels);
            }
        }
        sizes[u] = (counts[i] - last_count) as i32;
    }
}

#[test]
fn test() {
    let n = 7;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]];
    let labels = "abaedcd".to_string();
    let res = vec![2, 1, 1, 1, 1, 1, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
    let n = 4;
    let edges = vec_vec_i32![[0, 1], [1, 2], [0, 3]];
    let labels = "bbbb".to_string();
    let res = vec![4, 2, 1, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 3], [0, 4]];
    let labels = "aabab".to_string();
    let res = vec![3, 2, 1, 1, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 3], [0, 4]];
    let labels = "aabab".to_string();
    let res = vec![3, 2, 1, 1, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
    let n = 6;
    let edges = vec_vec_i32![[0, 1], [0, 2], [1, 3], [3, 4], [4, 5]];
    let labels = "cbabaa".to_string();
    let res = vec![1, 2, 1, 1, 2, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
    let n = 7;
    let edges = vec_vec_i32![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]];
    let labels = "aaabaaa".to_string();
    let res = vec![6, 5, 4, 1, 3, 2, 1];
    assert_eq!(Solution::count_sub_trees(n, edges, labels), res);
}

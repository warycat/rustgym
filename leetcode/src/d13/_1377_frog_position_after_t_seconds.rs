struct Solution;

impl Solution {
    fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        let mut prob: Vec<f64> = vec![0.0; n];
        prob[0] = 1.0;
        for e in edges {
            let u = e[0] as usize - 1;
            let v = e[1] as usize - 1;
            adj[u].push(v);
            adj[v].push(u);
        }
        Self::dfs(0, 0, t, &mut prob, &adj);
        prob[target as usize - 1]
    }
    fn dfs(u: usize, prev: usize, t: i32, prob: &mut Vec<f64>, adj: &[Vec<usize>]) {
        let size = adj[u].len() - if u == prev { 0 } else { 1 };
        if size == 0 || t == 0 {
            return;
        }
        let p = prob[u];
        for &v in &adj[u] {
            if v != prev {
                prob[v] += p / size as f64;
                Self::dfs(v, u, t - 1, prob, adj);
            }
        }
        prob[u] = 0.0;
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;

    let n = 7;
    let edges = vec_vec_i32![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]];
    let t = 2;
    let target = 4;
    let res = 0.16666666666666666;
    assert_approx_eq!(Solution::frog_position(n, edges, t, target), res);
    let n = 7;
    let edges = vec_vec_i32![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]];
    let t = 1;
    let target = 7;
    let res = 0.3333333333333333;
    assert_approx_eq!(Solution::frog_position(n, edges, t, target), res);
    let n = 7;
    let edges = vec_vec_i32![[1, 2], [1, 3], [1, 7], [2, 4], [2, 6], [3, 5]];
    let t = 20;
    let target = 6;
    let res = 0.16666666666666666;
    assert_approx_eq!(Solution::frog_position(n, edges, t, target), res);
}

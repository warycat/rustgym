struct Solution;

impl Solution {
    fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize - 1;
            let v = edge[1] as usize - 1;
            adj[u].push(v);
            adj[v].push(u);
        }
        let mut res = vec![0; n - 1];
        let full: u32 = (1 << n) as u32 - 1;
        for mask in 1..=full {
            let node_count = mask.count_ones() as usize;
            let mut edge_count = 0;
            for edge in &edges {
                let u = edge[0] as usize - 1;
                let v = edge[1] as usize - 1;
                if (mask & 1 << u) != 0 && (mask & 1 << v) != 0 {
                    edge_count += 1;
                }
            }
            if node_count == 1 || node_count != edge_count + 1 {
                continue;
            }
            let mut u = 0;
            for i in 0..n {
                if (mask & 1 << i) != 0 {
                    u = i;
                    break;
                }
            }
            let mut max = 0;
            let mut mask = mask;
            Self::dfs(u, &mut mask, &mut max, &adj, n);
            res[max - 1] += 1;
        }
        res
    }

    fn dfs(u: usize, mask: &mut u32, max: &mut usize, adj: &[Vec<usize>], n: usize) -> usize {
        *mask ^= 1 << u;
        let mut max_dia = 0;
        for &v in &adj[u] {
            if (*mask & 1 << v) != 0 {
                let dia = Self::dfs(v, mask, max, adj, n);
                *max = (*max).max(max_dia + dia);
                max_dia = max_dia.max(dia);
            }
        }
        max_dia + 1
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec_vec_i32![[1, 2], [2, 3], [2, 4]];
    let res = vec![3, 4, 0];
    assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), res);
    let n = 2;
    let edges = vec_vec_i32![[1, 2]];
    let res = vec![1];
    assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), res);
    let n = 3;
    let edges = vec_vec_i32![[1, 2], [2, 3]];
    let res = vec![2, 1];
    assert_eq!(Solution::count_subgraphs_for_each_diameter(n, edges), res);
}

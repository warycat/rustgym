struct Solution;
use std::collections::HashSet;

impl Solution {
    fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for e in richer {
            let u = e[0] as usize;
            let v = e[1] as usize;
            graph[v].insert(u);
        }
        let mut res = vec![n; n];
        for i in 0..n {
            Self::dfs(i, &mut res, &graph, &quiet, n);
        }
        res.into_iter().map(|x| x as i32).collect()
    }
    fn dfs(
        u: usize,
        stack: &mut Vec<usize>,
        graph: &[HashSet<usize>],
        quiet: &[i32],
        n: usize,
    ) -> usize {
        if stack[u] == n {
            stack[u] = u;
            for &v in &graph[u] {
                let w = Self::dfs(v, stack, graph, quiet, n);
                if quiet[w] < quiet[stack[u]] {
                    stack[u] = w;
                }
            }
        }
        stack[u]
    }
}

#[test]
fn test() {
    let richer = vec_vec_i32![[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]];
    let quiet = vec![3, 2, 5, 4, 6, 1, 7, 0];
    let res = vec![5, 5, 2, 5, 4, 5, 6, 7];
    assert_eq!(Solution::loud_and_rich(richer, quiet), res);
}

struct Solution;

impl Solution {
    fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut res = vec![];
        let mut state = vec![0; n];
        for i in 0..n {
            if Self::dfs(i, &mut state, &graph) {
                res.push(i as i32);
            }
        }
        res
    }

    fn dfs(u: usize, state: &mut [i32], graph: &[Vec<i32>]) -> bool {
        match state[u] {
            3 => false,
            2 => true,
            1 => {
                state[u] = 3;
                false
            }
            _ => {
                state[u] = 1;
                let mut s = 2;
                for &v in &graph[u] {
                    if !Self::dfs(v as usize, state, graph) {
                        s = 3;
                    }
                }
                state[u] = s;
                state[u] == 2
            }
        }
    }
}

#[test]
fn test() {
    let graph = vec_vec_i32![[1, 2], [2, 3], [5], [0], [5], [], []];
    let res = vec![2, 4, 5, 6];
    assert_eq!(Solution::eventual_safe_nodes(graph), res);
}

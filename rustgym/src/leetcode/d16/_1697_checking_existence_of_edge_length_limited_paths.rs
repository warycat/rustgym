struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind { parent, n }
    }

    fn find(&mut self, i: usize) -> usize {
        let j = self.parent[i];
        if j != i {
            let k = self.find(j);
            self.parent[i] = k;
            k
        } else {
            i
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
        }
    }
}

impl Solution {
    fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut edges: Vec<(i32, usize, usize)> = edge_list
            .into_iter()
            .map(|e| (e[2], e[0] as usize, e[1] as usize))
            .collect();
        edges.sort_unstable();
        edges.reverse();
        let mut queries: Vec<(i32, usize, usize, usize)> = queries
            .into_iter()
            .enumerate()
            .map(|(i, q)| (q[2], q[0] as usize, q[1] as usize, i))
            .collect();
        queries.sort_unstable();
        let m = queries.len();
        let mut res = vec![false; m];
        let mut uf = UnionFind::new(n);
        for (qd, qi, qj, qid) in queries {
            while let Some(&(ed, ei, ej)) = edges.last() {
                if ed < qd {
                    edges.pop();
                    uf.union(ei, ej);
                } else {
                    break;
                }
            }
            if uf.find(qi) == uf.find(qj) {
                res[qid] = true;
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let edge_list = vec_vec_i32![[0, 1, 2], [1, 2, 4], [2, 0, 8], [1, 0, 16]];
    let queries = vec_vec_i32![[0, 1, 2], [0, 2, 5]];
    let res = vec![false, true];
    assert_eq!(
        Solution::distance_limited_paths_exist(n, edge_list, queries),
        res
    );
    let n = 5;
    let edge_list = vec_vec_i32![[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]];
    let queries = vec_vec_i32![[0, 4, 14], [1, 4, 13]];
    let res = vec![true, false];
    assert_eq!(
        Solution::distance_limited_paths_exist(n, edge_list, queries),
        res
    );
}

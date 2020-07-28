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
        if i == j {
            i
        } else {
            let k = self.find(j);
            self.parent[i] = k;
            k
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            true
        } else {
            self.parent[i] = j;
            false
        }
    }
}

impl Solution {
    fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            if uf.union(u, v) {
                return edge;
            }
        }
        vec![]
    }
}

#[test]
fn test() {
    let edges = vec_vec_i32![[1, 2], [1, 3], [2, 3]];
    let res = vec![2, 3];
    assert_eq!(Solution::find_redundant_connection(edges), res);
    let edges = vec_vec_i32![[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]];
    let res = vec![1, 4];
    assert_eq!(Solution::find_redundant_connection(edges), res);
}

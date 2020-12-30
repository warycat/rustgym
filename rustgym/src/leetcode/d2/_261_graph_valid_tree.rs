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
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
            true
        } else {
            false
        }
    }
}

impl Solution {
    fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            if !uf.union(i, j) {
                return false;
            }
        }
        uf.n == 1
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [0, 2], [0, 3], [1, 4]];
    let res = true;
    assert_eq!(Solution::valid_tree(n, edges), res);
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [1, 2], [2, 3], [1, 3], [1, 4]];
    let res = false;
    assert_eq!(Solution::valid_tree(n, edges), res);
}

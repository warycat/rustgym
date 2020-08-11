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

    fn union(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}

impl Solution {
    fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            uf.union(i, j);
        }
        uf.n as i32
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [1, 2], [3, 4]];
    let res = 2;
    assert_eq!(Solution::count_components(n, edges), res);
    let n = 5;
    let edges = vec_vec_i32![[0, 1], [1, 2], [2, 3], [3, 4]];
    let res = 1;
    assert_eq!(Solution::count_components(n, edges), res);
}

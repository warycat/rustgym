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
    fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = connections.len();
        if m + 1 < n {
            return -1;
        }
        let mut uf = UnionFind::new(n);
        for connection in connections {
            let i = connection[0] as usize;
            let j = connection[1] as usize;
            uf.union(i, j);
        }
        (uf.n - 1) as i32
    }
}

#[test]
fn test() {
    let n = 4;
    let connections = vec_vec_i32![[0, 1], [0, 2], [1, 2]];
    let res = 1;
    assert_eq!(Solution::make_connected(n, connections), res);
    let n = 6;
    let connections = vec_vec_i32![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]];
    let res = 2;
    assert_eq!(Solution::make_connected(n, connections), res);
    let n = 6;
    let connections = vec_vec_i32![[0, 1], [0, 2], [0, 3], [1, 2]];
    let res = -1;
    assert_eq!(Solution::make_connected(n, connections), res);
    let n = 5;
    let connections = vec_vec_i32![[0, 1], [0, 2], [3, 4], [2, 3]];
    let res = 0;
    assert_eq!(Solution::make_connected(n, connections), res);
}

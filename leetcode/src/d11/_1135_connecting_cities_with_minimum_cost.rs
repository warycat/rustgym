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
        if j == i {
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
        }
        self.n -= 1;
    }
}

type Connection = (i32, usize, usize);

impl Solution {
    fn minimum_cost(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        let mut connections: Vec<Connection> = connections
            .into_iter()
            .map(|v| (v[2], v[0] as usize - 1, v[1] as usize - 1))
            .collect();
        connections.sort_unstable();
        let mut res = 0;
        for connection in connections {
            let u = connection.1;
            let v = connection.2;
            let i = uf.find(u);
            let j = uf.find(v);
            if i != j {
                uf.union(i, j);
                res += connection.0;
            }
        }
        if uf.n == 1 {
            res
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let n = 3;
    let connections = vec_vec_i32![[1, 2, 5], [1, 3, 6], [2, 3, 1]];
    let res = 6;
    assert_eq!(Solution::minimum_cost(n, connections), res);
    let n = 4;
    let connections = vec_vec_i32![[1, 2, 3], [3, 4, 4]];
    let res = -1;
    assert_eq!(Solution::minimum_cost(n, connections), res);
}

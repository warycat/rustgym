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
    fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let m = m as usize;
        let n = n as usize;
        let mut uf = UnionFind::new(m * n);
        let mut grid = vec![vec![0; n]; m];
        let mut group = 0;
        let mut res = vec![];
        for position in positions {
            let r = position[0] as usize;
            let c = position[1] as usize;
            let i = r * n + c;
            if grid[r][c] == 1 {
                res.push(group);
                continue;
            }
            grid[r][c] = 1;
            group += 1;
            if r > 0 && grid[r - 1][c] == 1 {
                let j = (r - 1) * n + c;
                if uf.union(i, j) {
                    group -= 1;
                }
            }
            if c > 0 && grid[r][c - 1] == 1 {
                let j = r * n + c - 1;
                if uf.union(i, j) {
                    group -= 1;
                }
            }
            if r + 1 < m && grid[r + 1][c] == 1 {
                let j = (r + 1) * n + c;
                if uf.union(i, j) {
                    group -= 1;
                }
            }
            if c + 1 < n && grid[r][c + 1] == 1 {
                let j = r * n + c + 1;
                if uf.union(i, j) {
                    group -= 1;
                }
            }
            res.push(group);
        }
        res
    }
}

#[test]
fn test() {
    let m = 3;
    let n = 3;
    let positions = vec_vec_i32![[0, 0], [0, 1], [1, 2], [2, 1]];
    let res = vec![1, 1, 2, 3];
    assert_eq!(Solution::num_islands2(m, n, positions), res);
}

pub struct Solution;

pub struct UnionFind {
    parents: Vec<usize>,
    n: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parents = (0..n).collect();
        UnionFind { parents, n }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let j = self.parents[i];
        if i == j {
            i
        } else {
            self.parents[i] = self.find(j);
            self.parents[i]
        }
    }

    pub fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parents[i] = j;
            self.n -= 1;
        }
    }
}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let a: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect()).collect();
        let n = grid.len();
        let m = a[0].len();
        let mut uf = UnionFind::new(n * m * 4);
        for i in 0..n {
            for j in 0..m {
                let k0 = Self::id(0, i, j, n, m);
                let k1 = Self::id(1, i, j, n, m);
                let k2 = Self::id(2, i, j, n, m);
                let k3 = Self::id(3, i, j, n, m);
                match a[i][j] {
                    ' ' => {
                        uf.union(k0, k1);
                        uf.union(k1, k2);
                        uf.union(k2, k3);
                        uf.union(k3, k0);
                    }
                    '/' => {
                        uf.union(k0, k1);
                        uf.union(k2, k3);
                    }
                    '\\' => {
                        uf.union(k1, k2);
                        uf.union(k3, k0);
                    }
                    _ => {}
                }
                if i > 0 {
                    uf.union(k1, Self::id(3, i - 1, j, n, m));
                }
                if j > 0 {
                    uf.union(k0, Self::id(2, i, j - 1, n, m));
                }
            }
        }
        uf.n as i32
    }

    pub fn id(k: usize, i: usize, j: usize, n: usize, m: usize) -> usize {
        k * n * m + i * m + j
    }
}

#[test]
fn test() {
    let grid = vec_string![" /", "/ "];
    let res = 2;
    assert_eq!(Solution::regions_by_slashes(grid), res);
    let grid = vec_string![" /", "  "];
    let res = 1;
    assert_eq!(Solution::regions_by_slashes(grid), res);
    let grid = vec_string!["\\/", "/\\"];
    let res = 4;
    assert_eq!(Solution::regions_by_slashes(grid), res);
}

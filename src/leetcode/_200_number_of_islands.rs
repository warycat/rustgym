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

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[i] = j;
            self.n -= 1;
        }
    }
}

impl Solution {
    fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }
        let m = grid[0].len();
        let mut uf = UnionFind::new(n * m + 1);
        for i in 0..n {
            for j in 0..m {
                let land = grid[i][j];
                if land == '1' {
                    if j > 0 && grid[i][j - 1] == '1' {
                        uf.union(i * m + j, i * m + j - 1);
                    }
                    if i > 0 && grid[i - 1][j] == '1' {
                        uf.union(i * m + j, (i - 1) * m + j);
                    }
                } else {
                    uf.union(i * m + j, n * m);
                }
            }
        }
        (uf.n - 1) as i32
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0']
    ];
    assert_eq!(Solution::num_islands(grid), 1);
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '0', '0', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '1', '0', '0'],
        ['0', '0', '0', '1', '1']
    ];
    assert_eq!(Solution::num_islands(grid), 3);
}

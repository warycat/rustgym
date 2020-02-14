struct Solution;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new() -> Self {
        UnionFind {
            parent: vec![],
            rank: vec![],
            count: 0,
        }
    }

    fn insert(&mut self, is_land: bool) -> usize {
        let key = self.rank.len();
        self.parent.push(key);
        self.rank.push(1);
        if is_land {
            self.count += 1;
        }
        key
    }

    fn find(&self, mut key: usize) -> usize {
        while self.parent[key] != key {
            key = self.parent[key];
        }
        key
    }

    fn union(&mut self, key_a: usize, key_b: usize) {
        let ka = self.find(key_a);
        let kb = self.find(key_b);
        if ka != kb {
            if self.rank[ka] < self.rank[kb] {
                self.parent[ka] = kb;
                self.rank[kb] += 1;
            } else {
                self.parent[kb] = ka;
                self.rank[ka] += 1;
            }
            self.count -= 1;
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
        let mut uf = UnionFind::new();
        for i in 0..n {
            for j in 0..m {
                let land = grid[i][j];
                uf.insert(land == '1');
                if land == '1' {
                    if j > 0 && grid[i][j - 1] == '1' {
                        uf.union(i * m + j, i * m + j - 1);
                    }
                    if i > 0 && grid[i - 1][j] == '1' {
                        uf.union(i * m + j, (i - 1) * m + j);
                    }
                }
            }
        }
        uf.count as i32
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

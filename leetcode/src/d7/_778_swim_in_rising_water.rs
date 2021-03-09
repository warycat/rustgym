struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
        }
    }
}

impl Solution {
    fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut uf = UnionFind::new(n * n);
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        for i in 0..n {
            for j in 0..n {
                queue.push((Reverse(grid[i][j]), i, j));
            }
        }
        while let Some((Reverse(t), r, c)) = queue.pop() {
            let i = r * n + c;
            if grid[r][c] <= t && r > 0 && grid[r - 1][c] <= t {
                let j = (r - 1) * n + c;
                uf.union(i, j);
            }
            if grid[r][c] <= t && r + 1 < n && grid[r + 1][c] <= t {
                let j = (r + 1) * n + c;
                uf.union(i, j);
            }
            if grid[r][c] <= t && c > 0 && grid[r][c - 1] <= t {
                let j = r * n + (c - 1);
                uf.union(i, j);
            }
            if grid[r][c] <= t && c + 1 < n && grid[r][c + 1] <= t {
                let j = r * n + (c + 1);
                uf.union(i, j);
            }
            if uf.find(0) == uf.find(n * n - 1) {
                return t;
            }
        }
        0
    }
}

#[test]
fn test() {
    let grid = vec_vec_i32![[0, 2], [1, 3]];
    let res = 3;
    assert_eq!(Solution::swim_in_water(grid), res);
    let grid = vec_vec_i32![
        [0, 1, 2, 3, 4],
        [24, 23, 22, 21, 5],
        [12, 13, 14, 15, 16],
        [11, 17, 18, 19, 20],
        [10, 9, 8, 7, 6]
    ];
    let res = 16;
    assert_eq!(Solution::swim_in_water(grid), res);
}

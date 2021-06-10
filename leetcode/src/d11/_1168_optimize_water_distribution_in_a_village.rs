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
        if i != j {
            let k = self.find(j);
            self.parent[i] = k;
            k
        } else {
            j
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let i = self.find(i);
        let j = self.find(j);
        if i != j {
            self.parent[i] = j;
            true
        } else {
            false
        }
    }
}

impl Solution {
    fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        let n = n as usize + 1;
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        for i in 0..n - 1 {
            queue.push((Reverse(wells[i]), 0, i + 1));
        }
        for pipe in pipes {
            queue.push((Reverse(pipe[2]), pipe[0] as usize, pipe[1] as usize));
        }
        let mut res = 0;
        let mut uf = UnionFind::new(n);
        while let Some(e) = queue.pop() {
            let u = e.1;
            let v = e.2;
            if uf.union(u, v) {
                res += (e.0).0;
            }
        }
        res
    }
}

#[test]
fn test() {
    let n = 3;
    let wells = vec![1, 2, 2];
    let pipes = vec_vec_i32![[1, 2, 1], [2, 3, 1]];
    let res = 3;
    assert_eq!(Solution::min_cost_to_supply_water(n, wells, pipes), res);
}

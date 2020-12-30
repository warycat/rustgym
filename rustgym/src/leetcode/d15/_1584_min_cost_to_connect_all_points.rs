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

    fn union(&mut self, mut i: usize, mut j: usize) -> bool {
        i = self.find(i);
        j = self.find(j);
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
    fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut queue: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();
        for i in 0..n {
            for j in i + 1..n {
                queue.push((Reverse(Self::dist(&points[i], &points[j])), i, j));
            }
        }
        let mut uf = UnionFind::new(n);
        let mut res = 0;
        while let Some((Reverse(d), i, j)) = queue.pop() {
            if uf.union(i, j) {
                res += d;
            }
            if uf.n == 1 {
                break;
            }
        }
        res
    }

    fn dist(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }
}

#[test]
fn test() {
    let points = vec_vec_i32![[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]];
    let res = 20;
    assert_eq!(Solution::min_cost_connect_points(points), res);
    let points = vec_vec_i32![[3, 12], [-2, 5], [-4, 1]];
    let res = 18;
    assert_eq!(Solution::min_cost_connect_points(points), res);
    let points = vec_vec_i32![[0, 0], [1, 1], [1, 0], [-1, 1]];
    let res = 4;
    assert_eq!(Solution::min_cost_connect_points(points), res);
    let points = vec_vec_i32![[-1000000, -1000000], [1000000, 1000000]];
    let res = 4000000;
    assert_eq!(Solution::min_cost_connect_points(points), res);
    let points = vec_vec_i32![[0, 0]];
    let res = 0;
    assert_eq!(Solution::min_cost_connect_points(points), res);
}

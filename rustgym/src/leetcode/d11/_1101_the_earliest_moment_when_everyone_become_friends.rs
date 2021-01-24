#![allow(clippy::unreadable_literal)]
struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct UnionFind {
    parent: Vec<usize>,
    group: usize,
    n: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind {
            parent,
            group: n,
            n,
        }
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

    fn union(&mut self, mut i: usize, mut j: usize) -> usize {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            self.parent[j] = i;
            self.group -= 1;
        }
        self.group
    }
}

type Log = (Reverse<i32>, usize, usize);

impl Solution {
    fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut queue: BinaryHeap<Log> = logs
            .iter()
            .map(|v| (Reverse(v[0]), v[1] as usize, v[2] as usize))
            .collect();
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        while let Some(log) = queue.pop() {
            if uf.union(log.1, log.2) == 1 {
                return (log.0).0;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let logs = vec_vec_i32![
        [20190101, 0, 1],
        [20190104, 3, 4],
        [20190107, 2, 3],
        [20190211, 1, 5],
        [20190224, 2, 4],
        [20190301, 0, 3],
        [20190312, 1, 2],
        [20190322, 4, 5]
    ];
    let n = 6;
    let res = 20190301;
    assert_eq!(Solution::earliest_acq(logs, n), res);
}

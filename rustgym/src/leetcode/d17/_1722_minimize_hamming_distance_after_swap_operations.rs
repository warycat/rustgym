struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

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
            j
        } else {
            self.parent[i] = self.find(j);
            self.parent[i]
        }
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);
        if i != j {
            let min = i.min(j);
            self.parent[i] = min;
            self.parent[j] = min;
        }
    }
}

impl Solution {
    fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut uf = UnionFind::new(n);
        for swap in allowed_swaps {
            let i = swap[0] as usize;
            let j = swap[1] as usize;
            uf.union(i, j);
        }
        let mut source_group: HashMap<usize, HashMap<i32, usize>> = HashMap::new();
        let mut target_group: HashMap<usize, HashMap<i32, usize>> = HashMap::new();
        let mut groups: HashMap<usize, HashSet<i32>> = HashMap::new();
        for i in 0..n {
            let g = uf.find(i);
            *source_group
                .entry(g)
                .or_default()
                .entry(source[i])
                .or_default() += 1;
            *target_group
                .entry(g)
                .or_default()
                .entry(target[i])
                .or_default() += 1;
            groups.entry(g).or_default().insert(source[i]);
            groups.entry(g).or_default().insert(target[i]);
        }
        let mut paired = 0;
        for (g, vals) in &groups {
            for x in vals {
                let s_cnt = source_group[g].get(x).unwrap_or(&0);
                let t_cnt = target_group[g].get(x).unwrap_or(&0);
                paired += s_cnt.min(t_cnt);
            }
        }
        (n - paired) as i32
    }
}

#[test]
fn test() {
    let source = vec![1, 2, 3, 4];
    let target = vec![2, 1, 4, 5];
    let allowed_swaps = vec_vec_i32![[0, 1], [2, 3]];
    let res = 1;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
    let source = vec![1, 2, 3, 4];
    let target = vec![1, 3, 2, 4];
    let allowed_swaps = vec_vec_i32![];
    let res = 2;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
    let source = vec![5, 1, 2, 4, 3];
    let target = vec![1, 5, 4, 2, 3];
    let allowed_swaps = vec_vec_i32![[0, 4], [4, 2], [1, 3], [1, 4]];
    let res = 0;
    assert_eq!(
        Solution::minimum_hamming_distance(source, target, allowed_swaps),
        res
    );
}

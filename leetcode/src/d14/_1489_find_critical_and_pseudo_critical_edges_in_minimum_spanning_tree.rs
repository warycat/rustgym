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
    fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = edges.len();
        let n = n as usize;
        let mut sorted_index: Vec<usize> = (0..m).collect();
        sorted_index.sort_unstable_by_key(|&i| edges[i][2]);
        let min_cost = Self::mst(std::usize::MAX, std::usize::MAX, &sorted_index, &edges, n);
        let mut critical = vec![];
        let mut noncritical = vec![];
        for i in 0..m {
            if Self::mst(i, std::usize::MAX, &sorted_index, &edges, n) > min_cost {
                critical.push(i as i32);
            } else {
                if Self::mst(std::usize::MAX, i, &sorted_index, &edges, n) == min_cost {
                    noncritical.push(i as i32);
                }
            }
        }
        vec![critical, noncritical]
    }

    fn mst(skip: usize, pick: usize, sorted_index: &[usize], edges: &[Vec<i32>], n: usize) -> i32 {
        let mut uf = UnionFind::new(n);
        let mut res = 0;
        if pick != std::usize::MAX {
            if uf.union(edges[pick][0] as usize, edges[pick][1] as usize) {
                res += edges[pick][2];
            }
        }
        for &idx in sorted_index {
            if idx != skip {
                if uf.union(edges[idx][0] as usize, edges[idx][1] as usize) {
                    res += edges[idx][2];
                }
            }
        }
        if uf.n == 1 {
            res
        } else {
            std::i32::MAX
        }
    }
}

#[test]
fn test() {
    let n = 5;
    let edges = vec_vec_i32![
        [0, 1, 1],
        [1, 2, 1],
        [2, 3, 2],
        [0, 3, 2],
        [0, 4, 3],
        [3, 4, 3],
        [1, 4, 6]
    ];
    let res = vec_vec_i32![[0, 1], [2, 3, 4, 5]];
    assert_eq!(
        Solution::find_critical_and_pseudo_critical_edges(n, edges),
        res
    );
    let n = 4;
    let edges = vec_vec_i32![[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]];
    let res = vec_vec_i32![[], [0, 1, 2, 3]];
    assert_eq!(
        Solution::find_critical_and_pseudo_critical_edges(n, edges),
        res
    );
}

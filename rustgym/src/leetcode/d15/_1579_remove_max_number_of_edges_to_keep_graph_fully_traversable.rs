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
        if i == j {
            false
        } else {
            self.n -= 1;
            self.parent[i] = j;
            true
        }
    }
}

impl Solution {
    fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let m = edges.len();
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut edges: Vec<(i32, usize, usize)> = edges
            .into_iter()
            .map(|v| (v[0], v[1] as usize - 1, v[2] as usize - 1))
            .collect();
        edges.sort_unstable();
        let mut keep = 0;
        while let Some(edge) = edges.pop() {
            match edge.0 {
                3 => {
                    let a = alice.union(edge.1, edge.2);
                    let b = bob.union(edge.1, edge.2);
                    if a || b {
                        keep += 1;
                    }
                }
                2 => {
                    if bob.union(edge.1, edge.2) {
                        keep += 1;
                    }
                }
                1 => {
                    if alice.union(edge.1, edge.2) {
                        keep += 1;
                    }
                }
                _ => {}
            }
        }
        if alice.n == 1 && bob.n == 1 {
            (m - keep) as i32
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let n = 4;
    let edges = vec_vec_i32![
        [3, 1, 2],
        [3, 2, 3],
        [1, 1, 3],
        [1, 2, 4],
        [1, 1, 2],
        [2, 3, 4]
    ];
    let res = 2;
    assert_eq!(Solution::max_num_edges_to_remove(n, edges), res);
    let n = 4;
    let edges = vec_vec_i32![[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]];
    let res = 0;
    assert_eq!(Solution::max_num_edges_to_remove(n, edges), res);
    let n = 4;
    let edges = vec_vec_i32![[3, 2, 3], [1, 1, 2], [2, 3, 4]];
    let res = -1;
    assert_eq!(Solution::max_num_edges_to_remove(n, edges), res);
}

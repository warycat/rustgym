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
    fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf = UnionFind::new(n as usize);
        for i in threshold + 1..n {
            let mut group = vec![];
            for j in 1..=n {
                if j % i == 0 {
                    group.push((j - 1) as usize);
                }
            }
            for w in group.windows(2) {
                uf.union(w[0], w[1]);
            }
        }
        let mut res = vec![];
        for q in queries {
            let a = uf.find((q[0] - 1) as usize);
            let b = uf.find((q[1] - 1) as usize);
            res.push(a == b)
        }
        res
    }
}

#[test]
fn test() {
    let n = 6;
    let threshold = 2;
    let queries = vec_vec_i32![[1, 4], [2, 5], [3, 6]];
    let res = vec![false, false, true];
    assert_eq!(Solution::are_connected(n, threshold, queries), res);
    let n = 6;
    let threshold = 0;
    let queries = vec_vec_i32![[4, 5], [3, 4], [3, 2], [2, 6], [1, 3]];
    let res = vec![true, true, true, true, true];
    assert_eq!(Solution::are_connected(n, threshold, queries), res);
    let n = 5;
    let threshold = 1;
    let queries = vec_vec_i32![[4, 5], [4, 5], [3, 2], [2, 3], [3, 4]];
    let res = vec![false, false, false, false, false];
    assert_eq!(Solution::are_connected(n, threshold, queries), res);
}

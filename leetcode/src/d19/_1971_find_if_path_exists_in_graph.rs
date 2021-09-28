struct Solution;

use rustgym_util::UnionFind;

impl Solution {
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            uf.union(u, v);
        }
        let start = start as usize;
        let end = end as usize;
        uf.find(start) == uf.find(end)
    }
}

#[test]
fn test() {
    let n = 3;
    let edges = vec_vec_i32![[0, 1], [1, 2], [2, 0]];
    let start = 0;
    let end = 2;
    let res = true;
    assert_eq!(Solution::valid_path(n, edges, start, end), res);
    let n = 6;
    let edges = vec_vec_i32![[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]];
    let start = 0;
    let end = 5;
    let res = false;
    assert_eq!(Solution::valid_path(n, edges, start, end), res);
}

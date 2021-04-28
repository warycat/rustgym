struct Solution;

use std::collections::HashSet;

impl Solution {
    fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut hs: HashSet<i32> = HashSet::new();
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            if !hs.insert(u) {
                return u;
            }
            if !hs.insert(v) {
                return v;
            }
        }
        0
    }
}

#[test]
fn test() {
    let edges = vec_vec_i32![[1, 2], [2, 3], [4, 2]];
    let res = 2;
    assert_eq!(Solution::find_center(edges), res);
    let edges = vec_vec_i32![[1, 2], [5, 1], [1, 3], [1, 4]];
    let res = 1;
    assert_eq!(Solution::find_center(edges), res);
}

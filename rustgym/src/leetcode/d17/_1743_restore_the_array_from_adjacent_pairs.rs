struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut nodes: HashSet<i32> = HashSet::new();
        for pair in adjacent_pairs {
            let u = pair[0];
            let v = pair[1];
            adj.entry(u).or_default().insert(v);
            adj.entry(v).or_default().insert(u);
            nodes.insert(u);
            nodes.insert(v);
        }
        let mut res = vec![];
        let mut u = *adj
            .iter()
            .filter(|(_, edges)| edges.len() == 1)
            .map(|(u, _)| u)
            .min()
            .unwrap();
        res.push(u);
        while adj[&u].len() == 1 {
            let v = *adj[&u].iter().next().unwrap();
            adj.get_mut(&u).unwrap().remove(&v);
            adj.get_mut(&v).unwrap().remove(&u);
            res.push(v);
            u = v;
        }
        res
    }
}

#[test]
fn test() {
    let adjacent_pairs = vec_vec_i32![[2, 1], [3, 4], [3, 2]];
    let res = vec![1, 2, 3, 4];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
    let adjacent_pairs = vec_vec_i32![[4, -2], [1, 4], [-3, 1]];
    let res = vec![-3, 1, 4, -2];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
    let adjacent_pairs = vec_vec_i32![[100000, -100000]];
    let res = vec![-100000, 100000];
    assert_eq!(Solution::restore_array(adjacent_pairs), res);
}

struct Solution;

use rustgym_util::*;
use std::collections::HashMap;
use std::collections::HashSet;

trait Preorder {
    fn preorder(&self, prev: i32, adj: &mut HashMap<i32, HashSet<i32>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, prev: i32, adj: &mut HashMap<i32, HashSet<i32>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if prev != -1 {
                adj.entry(prev).or_default().insert(val);
                adj.entry(val).or_default().insert(prev);
            }
            node.left.preorder(val, adj);
            node.right.preorder(val, adj);
        }
    }
}

impl Solution {
    fn find_distance(root: TreeLink, p: i32, q: i32) -> i32 {
        let mut adj: HashMap<i32, HashSet<i32>> = HashMap::new();
        root.preorder(-1, &mut adj);
        Self::dfs(p, -1, &adj, q)
    }
    fn dfs(cur: i32, prev: i32, adj: &HashMap<i32, HashSet<i32>>, q: i32) -> i32 {
        if cur == q {
            0
        } else {
            for &next in adj[&cur].iter() {
                if next != prev {
                    let dist = Self::dfs(next, cur, adj, q);
                    if dist != -1 {
                        return dist + 1;
                    }
                }
            }
            -1
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = 5;
    let q = 0;
    let res = 3;
    assert_eq!(Solution::find_distance(root, p, q), res);
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = 5;
    let q = 7;
    let res = 2;
    assert_eq!(Solution::find_distance(root, p, q), res);
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = 5;
    let q = 5;
    let res = 0;
    assert_eq!(Solution::find_distance(root, p, q), res);
}

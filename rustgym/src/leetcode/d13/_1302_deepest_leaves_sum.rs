struct Solution;
use rustgym_util::*;

trait Preorder {
    fn dfs(&self, level: usize, max: &mut usize, sum: &mut i32);
}

impl Preorder for TreeLink {
    fn dfs(&self, level: usize, max: &mut usize, sum: &mut i32) {
        use std::cmp::Ordering::*;
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            match level.cmp(max) {
                Greater => {
                    *max = level;
                    *sum = val;
                }
                Equal => {
                    *sum += val;
                }
                _ => {}
            }
            left.dfs(level + 1, max, sum);
            right.dfs(level + 1, max, sum);
        }
    }
}

impl Solution {
    fn deepest_leaves_sum(root: TreeLink) -> i32 {
        let mut res = 0;
        let mut max = 0;
        root.dfs(0, &mut max, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(2, tree!(4, tree!(7), None), tree!(5)),
        tree!(3, None, tree!(6, None, tree!(8)))
    );
    let res = 15;
    assert_eq!(Solution::deepest_leaves_sum(root), res);
}

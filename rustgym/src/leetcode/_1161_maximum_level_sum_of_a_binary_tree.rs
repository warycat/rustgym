struct Solution;
use rustgym_util::*;
use std::collections::HashMap;

trait Preorder {
    fn preorder(&self, level: usize, sum: &mut HashMap<usize, i32>);
}

impl Preorder for TreeLink {
    fn preorder(&self, level: usize, sum: &mut HashMap<usize, i32>) {
        if let Some(node) = self {
            let val = node.borrow().val;
            *sum.entry(level).or_default() += val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            left.preorder(level + 1, sum);
            right.preorder(level + 1, sum);
        }
    }
}

impl Solution {
    fn max_level_sum(root: TreeLink) -> i32 {
        let mut sum = HashMap::new();
        root.preorder(1, &mut sum);
        *sum.iter().max_by_key(|(_, &v)| v).unwrap().0 as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(7, tree!(7), tree!(-8)), tree!(0));
    let res = 2;
    assert_eq!(Solution::max_level_sum(root), res);
}

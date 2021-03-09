struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, leaves: &mut Vec<Vec<i32>>) -> usize;
}

impl Postorder for TreeLink {
    fn postorder(&self, leaves: &mut Vec<Vec<i32>>) -> usize {
        if let Some(node) = self {
            let left = node.borrow().left.postorder(leaves);
            let right = node.borrow().right.postorder(leaves);
            let level = left.max(right) + 1;
            if leaves.len() < level {
                leaves.push(vec![node.borrow().val]);
            } else {
                leaves[level - 1].push(node.borrow().val);
            }
            level
        } else {
            0
        }
    }
}

impl Solution {
    fn find_leaves(root: TreeLink) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        root.postorder(&mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    let res = vec![vec![4, 5, 3], vec![2], vec![1]];
    assert_eq!(Solution::find_leaves(root), res);
}

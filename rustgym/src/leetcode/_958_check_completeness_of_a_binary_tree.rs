struct Solution;
use rustgym_util::*;
use std::collections::HashSet;

trait Postorder {
    fn postorder(&self, id: u32, nodes: &mut HashSet<u32>) -> usize;
}

impl Postorder for TreeLink {
    fn postorder(&self, id: u32, nodes: &mut HashSet<u32>) -> usize {
        if let Some(node) = self {
            let node = node.borrow();
            let left = node.left.postorder(id << 1, nodes);
            let right = node.right.postorder((id << 1) | 1, nodes);
            nodes.insert(id);
            left + right + 1
        } else {
            0
        }
    }
}

impl Solution {
    fn is_complete_tree(root: TreeLink) -> bool {
        let mut nodes: HashSet<u32> = HashSet::new();
        let count = root.postorder(1, &mut nodes);
        nodes.len() == count && nodes.into_iter().all(|x| x <= count as u32)
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3, tree!(6), None));
    let res = true;
    assert_eq!(Solution::is_complete_tree(root), res);
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3, None, tree!(7)));
    let res = false;
    assert_eq!(Solution::is_complete_tree(root), res);
}

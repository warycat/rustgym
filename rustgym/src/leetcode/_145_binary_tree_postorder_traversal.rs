struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, nodes: &mut Vec<i32>);
}

impl Postorder for TreeLink {
    fn postorder(&self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.postorder(nodes);
            node.right.postorder(nodes);
            nodes.push(node.val);
        }
    }
}

impl Solution {
    fn postorder_traversal(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        root.postorder(&mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    let res = vec![3, 2, 1];
    assert_eq!(Solution::postorder_traversal(root), res);
}

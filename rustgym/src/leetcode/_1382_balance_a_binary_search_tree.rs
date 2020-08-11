struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(self, nodes: &mut Vec<i32>);
    fn build(nodes: &[i32]) -> TreeLink;
}

impl Inorder for TreeLink {
    fn inorder(self, nodes: &mut Vec<i32>) {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            let val = node.val;
            let left = node.left.take();
            let right = node.right.take();
            left.inorder(nodes);
            nodes.push(val);
            right.inorder(nodes);
        }
    }
    fn build(nodes: &[i32]) -> TreeLink {
        let n = nodes.len();
        if n == 0 {
            None
        } else {
            let m = n / 2;
            tree!(
                nodes[m],
                TreeLink::build(&nodes[..m]),
                TreeLink::build(&nodes[m + 1..])
            )
        }
    }
}

impl Solution {
    fn balance_bst(root: TreeLink) -> TreeLink {
        let mut nodes: Vec<i32> = vec![];
        root.inorder(&mut nodes);
        TreeLink::build(&nodes)
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, None, tree!(3, None, tree!(4))));
    let res = tree!(3, tree!(2, tree!(1), None), tree!(4));
    assert_eq!(Solution::balance_bst(root), res);
}

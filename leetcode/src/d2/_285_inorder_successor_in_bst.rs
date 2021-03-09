struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, p: i32, successor: &mut TreeLink);
}

impl Inorder for TreeLink {
    fn inorder(&self, p: i32, successor: &mut TreeLink) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.inorder(p, successor);
            if successor.is_none() && node.val > p {
                *successor = tree!(node.val);
            }
            node.right.inorder(p, successor);
        }
    }
}

impl Solution {
    fn inorder_successor(root: TreeLink, p: TreeLink) -> TreeLink {
        let p = p.as_ref().unwrap().borrow().val;
        let mut res = None;
        root.inorder(p, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(1), tree!(3));
    let p = tree!(1);
    let res = tree!(2);
    assert_eq!(Solution::inorder_successor(root, p), res);
    let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
    let p = tree!(6);
    let res = None;
    assert_eq!(Solution::inorder_successor(root, p), res);
}

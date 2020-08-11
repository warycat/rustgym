struct Solution;
use rustgym_util::*;

trait Postortder {
    fn split(self, v: i32) -> (TreeLink, TreeLink);
}

impl Postortder for TreeLink {
    fn split(self, v: i32) -> (TreeLink, TreeLink) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if val <= v {
                let (less, greater) = right.split(v);
                node.borrow_mut().left = left;
                node.borrow_mut().right = less;
                (Some(node), greater)
            } else {
                let (less, greater) = left.split(v);
                node.borrow_mut().left = greater;
                node.borrow_mut().right = right;
                (less, Some(node))
            }
        } else {
            (None, None)
        }
    }
}

impl Solution {
    fn split_bst(root: TreeLink, v: i32) -> Vec<TreeLink> {
        let (l, r) = root.split(v);
        vec![l, r]
    }
}

#[test]
fn test() {
    let root = tree!(
        4,
        tree!(2, tree!(1), tree!(3)),
        tree!(6, tree!(5), tree!(7))
    );
    let v = 2;
    let res = vec![
        tree!(2, tree!(1), None),
        tree!(4, tree!(3), tree!(6, tree!(5), tree!(7))),
    ];
    assert_eq!(Solution::split_bst(root, v), res);
}

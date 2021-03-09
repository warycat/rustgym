struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self) -> (usize, TreeLink);
}

impl Postorder for TreeLink {
    fn postorder(&self) -> (usize, TreeLink) {
        use std::cmp::Ordering::*;
        if let Some(node) = self {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let (height_l, left_lca) = left.postorder();
            let (height_r, right_lca) = right.postorder();
            match height_l.cmp(&height_r) {
                Less => (height_r + 1, right_lca),
                Greater => (height_l + 1, left_lca),
                Equal => (height_l + 1, self.clone()),
            }
        } else {
            (0, None)
        }
    }
}

impl Solution {
    fn lca_deepest_leaves(root: TreeLink) -> TreeLink {
        let (_, lca) = root.postorder();
        lca
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    let res = tree!(1, tree!(2), tree!(3));
    assert_eq!(Solution::lca_deepest_leaves(root), res);
    let root = tree!(1, tree!(2, tree!(4), None), tree!(3));
    let res = tree!(4);
    assert_eq!(Solution::lca_deepest_leaves(root), res);
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    let res = tree!(2, tree!(4), tree!(5));
    assert_eq!(Solution::lca_deepest_leaves(root), res);
    let root = tree!(
        1,
        tree!(2, tree!(3, None, tree!(6)), tree!(4, None, tree!(5))),
        None
    );
    let res = tree!(2, tree!(3, None, tree!(6)), tree!(4, None, tree!(5)));
    assert_eq!(Solution::lca_deepest_leaves(root), res);
}

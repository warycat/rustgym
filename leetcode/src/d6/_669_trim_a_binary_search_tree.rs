struct Solution;
use rustgym_util::*;

impl Solution {
    fn trim_bst(root: TreeLink, l: i32, r: i32) -> TreeLink {
        if let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            if node.val > r {
                return Self::trim_bst(left, l, r);
            }
            if node.val < l {
                return Self::trim_bst(right, l, r);
            }
            node.left = Self::trim_bst(left, l, r);
            node.right = Self::trim_bst(right, l, r);
            root
        } else {
            None
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(0), tree!(2));
    let res = tree!(1, None, tree!(2));
    assert_eq!(Solution::trim_bst(root, 1, 2), res);
    let root = tree!(3, tree!(0, None, tree!(2, tree!(1), None)), tree!(4));
    let res = tree!(3, tree!(2, tree!(1), None), None);
    assert_eq!(Solution::trim_bst(root, 1, 3), res);
}

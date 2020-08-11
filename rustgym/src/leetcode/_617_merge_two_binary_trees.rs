struct Solution;
use rustgym_util::*;

impl Solution {
    fn merge_trees(t1: TreeLink, t2: TreeLink) -> TreeLink {
        match (t1, t2) {
            (Some(n1), Some(n2)) => {
                let mut n1 = n1.borrow_mut();
                let mut n2 = n2.borrow_mut();
                tree!(
                    n1.val + n2.val,
                    Self::merge_trees(n1.left.take(), n2.left.take()),
                    Self::merge_trees(n1.right.take(), n2.right.take())
                )
            }
            (None, Some(n2)) => Some(n2),
            (Some(n1), None) => Some(n1),
            (None, None) => None,
        }
    }
}

#[test]
fn test() {
    let t1 = tree!(1, tree!(3, tree!(5), None), tree!(2));
    let t2 = tree!(2, tree!(1, None, tree!(4)), tree!(3, None, tree!(7)));
    let res = tree!(3, tree!(4, tree!(5), tree!(4)), tree!(5, None, tree!(7)));
    assert_eq!(Solution::merge_trees(t1, t2), res);
}

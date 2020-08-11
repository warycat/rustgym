struct Solution;
use rustgym_util::*;

trait Count {
    fn count(&self) -> i32;
}

impl Count for TreeLink {
    fn count(&self) -> i32 {
        if let Some(node) = self {
            1 + node.borrow().left.count() + node.borrow().right.count()
        } else {
            0
        }
    }
}

impl Solution {
    fn count_nodes(root: TreeLink) -> i32 {
        root.count()
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3, tree!(6), None));
    let res = 6;
    assert_eq!(Solution::count_nodes(root), res);
}

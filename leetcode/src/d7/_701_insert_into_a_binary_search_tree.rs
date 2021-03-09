struct Solution;
use rustgym_util::*;

trait Postorder {
    fn insert(self, val: i32) -> Self;
}

impl Postorder for TreeLink {
    fn insert(self, val: i32) -> Self {
        if let Some(node) = self {
            let node_val = node.borrow().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if node_val < val {
                tree!(node_val, left, right.insert(val))
            } else {
                tree!(node_val, left.insert(val), right)
            }
        } else {
            tree!(val)
        }
    }
}

impl Solution {
    fn insert_into_bst(root: TreeLink, val: i32) -> TreeLink {
        root.insert(val)
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(7));
    let val = 5;
    let res = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(7, tree!(5), None));
    assert_eq!(Solution::insert_into_bst(root, val), res);
}

struct Solution;
use rustgym_util::*;

trait Postorder {
    fn insert(self, val: i32) -> Self;
}

impl Postorder for TreeLink {
    fn insert(self, val: i32) -> Self {
        if let Some(node) = self {
            let node_val = node.borrow().val;
            if node_val < val {
                tree!(val, Some(node), None)
            } else {
                let right = node.borrow_mut().right.take();
                node.borrow_mut().right = right.insert(val);
                Some(node)
            }
        } else {
            tree!(val)
        }
    }
}

impl Solution {
    fn insert_into_max_tree(root: TreeLink, val: i32) -> TreeLink {
        root.insert(val)
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(1), tree!(3, tree!(2), None));
    let val = 5;
    let res = tree!(5, tree!(4, tree!(1), tree!(3, tree!(2), None)), None);
    assert_eq!(Solution::insert_into_max_tree(root, val), res);
    let root = tree!(5, tree!(2, None, tree!(1)), tree!(4));
    let val = 3;
    let res = tree!(5, tree!(2, None, tree!(1)), tree!(4, None, tree!(3)));
    assert_eq!(Solution::insert_into_max_tree(root, val), res);
    let root = tree!(5, tree!(2, None, tree!(1)), tree!(3));
    let val = 4;
    let res = tree!(5, tree!(2, None, tree!(1)), tree!(4, tree!(3), None));
    assert_eq!(Solution::insert_into_max_tree(root, val), res);
}

struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self, depth: i32, v: i32, d: i32) -> TreeLink;
}

impl Postorder for TreeLink {
    fn postorder(self, depth: i32, v: i32, d: i32) -> TreeLink {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            let val = node.val;
            let left = node.left.take();
            let right = node.right.take();
            let mut left = left.postorder(depth + 1, v, d);
            let mut right = right.postorder(depth + 1, v, d);
            if depth + 1 == d {
                left = tree!(v, left, None);
                right = tree!(v, None, right);
            }
            tree!(val, left, right)
        } else {
            None
        }
    }
}

impl Solution {
    fn add_one_row(root: TreeLink, v: i32, d: i32) -> TreeLink {
        if d == 1 {
            tree!(v, root, None)
        } else {
            root.postorder(1, v, d)
        }
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(3), tree!(1)), tree!(6, tree!(5), None));
    let v = 1;
    let d = 2;
    let res = tree!(
        4,
        tree!(1, tree!(2, tree!(3), tree!(1)), None),
        tree!(1, None, tree!(6, tree!(5), None))
    );
    assert_eq!(Solution::add_one_row(root, v, d), res);
    let root = tree!(4, tree!(2, tree!(3), tree!(1)), None);
    let v = 1;
    let d = 3;
    let res = tree!(
        4,
        tree!(2, tree!(1, tree!(3), None), tree!(1, None, tree!(1))),
        None
    );
    assert_eq!(Solution::add_one_row(root, v, d), res);
}

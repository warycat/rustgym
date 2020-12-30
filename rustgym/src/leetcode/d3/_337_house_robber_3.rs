struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self) -> (i32, i32);
}

impl Postorder for TreeLink {
    fn postorder(&self) -> (i32, i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.postorder();
            let right = node.right.postorder();
            (
                val + left.1 + right.1,
                left.0.max(left.1) + right.0.max(right.1),
            )
        } else {
            (0, 0)
        }
    }
}

impl Solution {
    fn rob(root: TreeLink) -> i32 {
        let (a, b) = root.postorder();
        a.max(b)
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(2, None, tree!(3)), tree!(3, None, tree!(1)));
    let res = 7;
    assert_eq!(Solution::rob(root), res);
    let root = tree!(3, tree!(4, tree!(1), tree!(3)), tree!(5, None, tree!(1)));
    let res = 9;
    assert_eq!(Solution::rob(root), res);
}

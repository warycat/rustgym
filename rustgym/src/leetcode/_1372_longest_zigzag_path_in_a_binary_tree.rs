struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, max: &mut i32) -> (i32, i32);
}

impl Postorder for TreeLink {
    fn postorder(&self, max: &mut i32) -> (i32, i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let (_, left_right) = node.left.postorder(max);
            let (right_left, _) = node.right.postorder(max);
            let left = left_right + 1;
            let right = right_left + 1;
            *max = (*max).max(left);
            *max = (*max).max(right);
            (left, right)
        } else {
            (0, 0)
        }
    }
}

impl Solution {
    fn longest_zig_zag(root: TreeLink) -> i32 {
        let mut res = 0;
        root.postorder(&mut res);
        res - 1
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        None,
        tree!(
            1,
            tree!(1),
            tree!(1, tree!(1, None, tree!(1, None, tree!(1))), tree!(1))
        )
    );
    let res = 3;
    assert_eq!(Solution::longest_zig_zag(root), res);
    let root = tree!(
        1,
        tree!(1, None, tree!(1, tree!(1, None, tree!(1)), tree!(1))),
        tree!(1)
    );
    let res = 4;
    assert_eq!(Solution::longest_zig_zag(root), res);
    let root = tree!(1);
    let res = 0;
    assert_eq!(Solution::longest_zig_zag(root), res);
}

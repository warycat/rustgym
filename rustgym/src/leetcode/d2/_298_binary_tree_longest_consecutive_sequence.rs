struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, max: &mut usize) -> Option<(i32, usize)>;
}

impl Postorder for TreeLink {
    fn postorder(&self, max: &mut usize) -> Option<(i32, usize)> {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let mut length = 1;
            if let Some(left) = node.left.postorder(max) {
                if val + 1 == left.0 {
                    length = length.max(left.1 + 1);
                }
            }
            if let Some(right) = node.right.postorder(max) {
                if val + 1 == right.0 {
                    length = length.max(right.1 + 1);
                }
            }
            *max = (*max).max(length);
            Some((val, length))
        } else {
            None
        }
    }
}

impl Solution {
    fn longest_consecutive(root: TreeLink) -> i32 {
        let mut res = 0;
        root.postorder(&mut res);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(3, tree!(2), tree!(4, None, tree!(5))));
    let res = 3;
    assert_eq!(Solution::longest_consecutive(root), res);
    let root = tree!(2, None, tree!(3, tree!(2, tree!(1), None), None));
    let res = 2;
    assert_eq!(Solution::longest_consecutive(root), res);
}

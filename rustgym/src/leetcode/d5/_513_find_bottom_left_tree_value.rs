struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, height: usize, leftmost: &mut (usize, i32));
}

impl Preorder for TreeLink {
    fn preorder(&self, height: usize, leftmost: &mut (usize, i32)) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            if height > leftmost.0 {
                *leftmost = (height, val);
            }
            left.preorder(height + 1, leftmost);
            right.preorder(height + 1, leftmost);
        }
    }
}

impl Solution {
    fn find_bottom_left_value(root: TreeLink) -> i32 {
        let mut leftmost: (usize, i32) = (0, 0);
        root.preorder(1, &mut leftmost);
        leftmost.1
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(1), tree!(3));
    let res = 1;
    assert_eq!(Solution::find_bottom_left_value(root), res);
    let root = tree!(
        1,
        tree!(2, tree!(4), None),
        tree!(3, tree!(5, tree!(7), None), tree!(6))
    );
    let res = 7;
    assert_eq!(Solution::find_bottom_left_value(root), res);
}

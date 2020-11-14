struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, prev: &mut Option<i32>, min: &mut i32);
}

impl Inorder for TreeLink {
    fn inorder(&self, prev: &mut Option<i32>, min: &mut i32) {
        if let Some(node) = self {
            let node = node.borrow();
            Self::inorder(&node.left, prev, min);
            if let Some(prev_val) = prev.as_mut() {
                *min = (node.val - *prev_val).abs().min(*min);
                *prev_val = node.val;
            } else {
                *prev = Some(node.val);
            }
            Self::inorder(&node.right, prev, min);
        }
    }
}

impl Solution {
    fn get_minimum_difference(root: TreeLink) -> i32 {
        let mut min = i32::MAX;
        let mut prev: Option<i32> = None;
        root.inorder(&mut prev, &mut min);
        min
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(3, tree!(2), None));
    assert_eq!(Solution::get_minimum_difference(root), 1);
}

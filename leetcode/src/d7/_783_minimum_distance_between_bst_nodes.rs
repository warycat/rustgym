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
                *min = i32::min((node.val - *prev_val).abs(), *min);
                *prev_val = node.val;
            } else {
                *prev = Some(node.val);
            }
            Self::inorder(&node.right, prev, min);
        }
    }
}

impl Solution {
    fn min_diff_in_bst(root: TreeLink) -> i32 {
        let mut min = std::i32::MAX;
        let mut prev: Option<i32> = None;
        root.inorder(&mut prev, &mut min);
        min
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(6));
    assert_eq!(Solution::min_diff_in_bst(root), 1);
}

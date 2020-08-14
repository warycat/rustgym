struct Solution;
use rustgym_util::*;

trait Diameter {
    fn diameter(&self) -> i32;
    fn max_depth(&self, max: &mut i32) -> i32;
}

impl Diameter for TreeLink {
    fn diameter(&self) -> i32 {
        let mut max: i32 = 0;
        self.max_depth(&mut max);
        max
    }

    fn max_depth(&self, max: &mut i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = node.left.max_depth(max);
            let right = node.right.max_depth(max);
            *max = (*max).max(left + right);
            (left + 1).max(right + 1)
        } else {
            0
        }
    }
}

impl Solution {
    fn diameter_of_binary_tree(root: TreeLink) -> i32 {
        root.diameter()
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}

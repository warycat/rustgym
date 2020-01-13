struct Solution;
use crate::util::*;

trait Height {
    fn height(&self) -> usize;
}

impl Height for TreeLink {
    fn height(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                usize::max(node.left.height(), node.right.height()) + 1
            }
        }
    }
}

impl Solution {
    fn is_balanced_r(root: &TreeLink) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left = &node.left;
                let right = &node.right;
                let height_left = left.height();
                let height_right = right.height();
                if height_left == height_right
                    || height_left == height_right + 1
                    || height_left + 1 == height_right
                {
                    Solution::is_balanced_r(left) && Solution::is_balanced_r(right)
                } else {
                    false
                }
            }
        }
    }
    fn is_balanced(root: TreeLink) -> bool {
        Solution::is_balanced_r(&root)
    }
}

#[test]
fn test() {
    let a = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::is_balanced(a), true);
    let b = tree!(
        1,
        tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
        tree!(2)
    );
    assert_eq!(Solution::is_balanced(b), false);
}

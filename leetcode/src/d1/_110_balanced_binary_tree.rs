struct Solution;
use rustgym_util::*;

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

trait Balanced {
    fn is_balanced(&self) -> bool;
}

impl Balanced for TreeLink {
    fn is_balanced(&self) -> bool {
        match self {
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
                    left.is_balanced() && right.is_balanced()
                } else {
                    false
                }
            }
        }
    }
}

impl Solution {
    fn is_balanced(root: TreeLink) -> bool {
        root.is_balanced()
    }
}

#[test]
fn test() {
    let a = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = true;
    assert_eq!(Solution::is_balanced(a), res);
    let b = tree!(
        1,
        tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
        tree!(2)
    );
    let res = false;
    assert_eq!(Solution::is_balanced(b), res);
}

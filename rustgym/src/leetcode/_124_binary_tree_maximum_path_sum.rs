struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, max: &mut i32) -> Option<i32>;
}

impl Postorder for TreeLink {
    fn postorder(&self, max: &mut i32) -> Option<i32> {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            let left_max = left.postorder(max);
            let right_max = right.postorder(max);
            match (left_max, right_max) {
                (Some(left_max), Some(right_max)) => {
                    *max = (*max).max(val);
                    *max = (*max).max(val + left_max);
                    *max = (*max).max(val + right_max);
                    *max = (*max).max(val + left_max + right_max);
                    Some(val + 0.max(left_max.max(right_max)))
                }
                (Some(left_max), None) => {
                    *max = (*max).max(val);
                    *max = (*max).max(val + left_max);
                    Some(val + 0.max(left_max))
                }
                (None, Some(right_max)) => {
                    *max = (*max).max(val);
                    *max = (*max).max(val + right_max);
                    Some(val + 0.max(right_max))
                }
                (None, None) => {
                    *max = (*max).max(val);
                    Some(val)
                }
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn max_path_sum(root: TreeLink) -> i32 {
        let mut res: i32 = std::i32::MIN;
        root.postorder(&mut res);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    let res = 6;
    assert_eq!(Solution::max_path_sum(root), res);
    let root = tree!(-10, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = 42;
    assert_eq!(Solution::max_path_sum(root), res);
}

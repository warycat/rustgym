struct Solution;
use rustgym_util::*;

trait SumOfLeftLeaves {
    fn sum_of_left_leaves(&self) -> i32;
}

impl SumOfLeftLeaves for TreeLink {
    fn sum_of_left_leaves(&self) -> i32 {
        let mut sum = 0;
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if let Some(left_node) = left {
                let left_node = left_node.borrow();
                if left_node.left.is_none() && left_node.right.is_none() {
                    sum += left_node.val;
                } else {
                    sum += Self::sum_of_left_leaves(left);
                }
            }
            sum += Self::sum_of_left_leaves(right);
        }
        sum
    }
}

impl Solution {
    fn sum_of_left_leaves(root: TreeLink) -> i32 {
        root.sum_of_left_leaves()
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    assert_eq!(Solution::sum_of_left_leaves(root), 24);
}

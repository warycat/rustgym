struct Solution;
use rustgym_util::*;

impl Solution {
    fn sum_root_to_leaf(root: TreeLink) -> i32 {
        let mut sum = 0;
        Self::sum_r(&root, 0, &mut sum);
        sum
    }
    fn sum_r(link: &TreeLink, parent_val: i32, sum: &mut i32) {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val + parent_val * 2;
            if left.is_none() && right.is_none() {
                *sum += val;
            } else {
                Self::sum_r(left, val, sum);
                Self::sum_r(right, val, sum);
            }
        }
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(0, tree!(0), tree!(1)),
        tree!(1, tree!(0), tree!(1))
    );
    assert_eq!(Solution::sum_root_to_leaf(root), 22);
}

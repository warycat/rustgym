struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, max: &mut f64) -> (i32, i32);
}

impl Postorder for TreeLink {
    fn postorder(&self, max: &mut f64) -> (i32, i32) {
        let res = if let Some(node) = self {
            let val = node.borrow().val;
            let (sum_l, size_l) = node.borrow().left.postorder(max);
            let (sum_r, size_r) = node.borrow().right.postorder(max);
            (sum_l + sum_r + val, size_l + size_r + 1)
        } else {
            (0, 0)
        };
        *max = (*max).max(res.0 as f64 / res.1 as f64);
        res
    }
}

impl Solution {
    fn maximum_average_subtree(root: TreeLink) -> f64 {
        let mut res = std::f64::MIN;
        root.postorder(&mut res);
        res
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let root = tree!(5, tree!(6), tree!(1));
    let res = 6.0;
    assert_approx_eq!(Solution::maximum_average_subtree(root), res);
}

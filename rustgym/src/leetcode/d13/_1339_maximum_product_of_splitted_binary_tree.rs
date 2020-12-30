struct Solution;
use rustgym_util::*;

trait Postorder {
    fn sum(&self) -> i32;
    fn product(&self, max: &mut i64, sum: i32) -> i32;
}

impl Postorder for TreeLink {
    fn sum(&self) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.sum();
            let right = node.right.sum();
            val + left + right
        } else {
            0
        }
    }

    fn product(&self, max: &mut i64, sum: i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.product(max, sum);
            let right = node.right.product(max, sum);
            let res = val + left + right;
            *max = (*max).max((sum - res) as i64 * res as i64);
            res
        } else {
            0
        }
    }
}

impl Solution {
    fn max_product(root: TreeLink) -> i32 {
        let sum = root.sum();
        let mut res = 0;
        root.product(&mut res, sum);
        (res % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3, tree!(6), None));
    let res = 110;
    assert_eq!(Solution::max_product(root), res);
    let root = tree!(1, None, tree!(2, tree!(3), tree!(4, tree!(5), tree!(6))));
    let res = 90;
    assert_eq!(Solution::max_product(root), res);
    let root = tree!(1, tree!(1), None);
    let res = 1;
    assert_eq!(Solution::max_product(root), res);
}

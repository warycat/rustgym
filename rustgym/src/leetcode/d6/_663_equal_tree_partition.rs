struct Solution;
use rustgym_util::*;

trait Postorder {
    fn sum_size(&self) -> (i32, usize);
    fn check(&self, found: &mut bool, sum: i32, size: usize) -> (i32, usize);
}

impl Postorder for TreeLink {
    fn sum_size(&self) -> (i32, usize) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.sum_size();
            let right = node.right.sum_size();
            (val + left.0 + right.0, 1 + left.1 + right.1)
        } else {
            (0, 0)
        }
    }
    fn check(&self, found: &mut bool, sum: i32, size: usize) -> (i32, usize) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.check(found, sum, size);
            let right = node.right.check(found, sum, size);
            let res = (val + left.0 + right.0, 1 + left.1 + right.1);
            if res.0 * 2 == sum && res.1 != size {
                *found = true;
            }
            res
        } else {
            (0, 0)
        }
    }
}

impl Solution {
    fn check_equal_tree(root: TreeLink) -> bool {
        let (sum, size) = root.sum_size();
        let mut res = false;
        root.check(&mut res, sum, size);
        res
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(10), tree!(10, tree!(2), tree!(3)));
    let res = true;
    assert_eq!(Solution::check_equal_tree(root), res);
}

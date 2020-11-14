struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, path: i32, sum: &mut i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, path: i32, sum: &mut i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let path = path * 10 + val;
            if node.left.is_none() && node.right.is_none() {
                *sum += path;
            }
            node.left.preorder(path, sum);
            node.right.preorder(path, sum);
        }
    }
}

impl Solution {
    fn sum_numbers(root: TreeLink) -> i32 {
        let mut res = 0;
        root.preorder(0, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    let res = 25;
    assert_eq!(Solution::sum_numbers(root), res);
    let root = tree!(4, tree!(9, tree!(5), tree!(1)), tree!(0));
    let res = 1026;
    assert_eq!(Solution::sum_numbers(root), res);
}

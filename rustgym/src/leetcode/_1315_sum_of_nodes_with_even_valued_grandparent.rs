struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, parent: bool, grand_parent: bool, sum: &mut i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, parent: bool, grand_parent: bool, sum: &mut i32) {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            if grand_parent {
                *sum += val;
            }
            left.preorder(val % 2 == 0, parent, sum);
            right.preorder(val % 2 == 0, parent, sum);
        }
    }
}

impl Solution {
    fn sum_even_grandparent(root: TreeLink) -> i32 {
        let mut res = 0;
        root.preorder(false, false, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        6,
        tree!(7, tree!(2, tree!(9), None), tree!(7, tree!(1), tree!(4))),
        tree!(8, tree!(1), tree!(3, None, tree!(5)))
    );
    let res = 18;
    assert_eq!(Solution::sum_even_grandparent(root), res);
}

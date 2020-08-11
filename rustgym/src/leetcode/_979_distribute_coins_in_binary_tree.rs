struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, sum: &mut i32) -> i32;
}

impl Postorder for TreeLink {
    fn postorder(&self, sum: &mut i32) -> i32 {
        if let Some(node) = self {
            let val = node.borrow().val;
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            let l = left.postorder(sum);
            let r = right.postorder(sum);
            let m = val + l + r - 1;
            *sum += m.abs();
            m
        } else {
            0
        }
    }
}

impl Solution {
    fn distribute_coins(root: TreeLink) -> i32 {
        let mut res = 0;
        root.postorder(&mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(0), tree!(0));
    let res = 2;
    assert_eq!(Solution::distribute_coins(root), res);
    let root = tree!(0, tree!(3), tree!(0));
    let res = 3;
    assert_eq!(Solution::distribute_coins(root), res);
    let root = tree!(1, tree!(0), tree!(2));
    let res = 2;
    assert_eq!(Solution::distribute_coins(root), res);
    let root = tree!(1, tree!(0, None, tree!(3)), tree!(0));
    let res = 4;
    assert_eq!(Solution::distribute_coins(root), res);
}

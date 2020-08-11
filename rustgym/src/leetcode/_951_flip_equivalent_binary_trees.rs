struct Solution;
use rustgym_util::*;

trait FlipEq {
    fn flip_eq(root1: &TreeLink, root2: &TreeLink) -> bool;
}

impl FlipEq for TreeLink {
    fn flip_eq(root1: &TreeLink, root2: &TreeLink) -> bool {
        if let (Some(node1), Some(node2)) = (root1, root2) {
            let val1 = node1.borrow().val;
            let val2 = node2.borrow().val;
            let left1 = &node1.borrow().left;
            let right1 = &node1.borrow().right;
            let left2 = &node2.borrow().left;
            let right2 = &node2.borrow().right;
            val1 == val2
                && (Self::flip_eq(left1, left2) && Self::flip_eq(right1, right2)
                    || Self::flip_eq(left1, right2) && Self::flip_eq(right1, left2))
        } else {
            root1 == root2
        }
    }
}

impl Solution {
    fn flip_equiv(root1: TreeLink, root2: TreeLink) -> bool {
        TreeLink::flip_eq(&root1, &root2)
    }
}

#[test]
fn test() {
    let root1 = tree!(
        1,
        tree!(2, tree!(4), tree!(5, tree!(7), tree!(8))),
        tree!(3, tree!(6), None)
    );
    let root2 = tree!(
        1,
        tree!(3, None, tree!(6)),
        tree!(2, tree!(4), tree!(5, tree!(8), tree!(7)))
    );
    let res = true;
    assert_eq!(Solution::flip_equiv(root1, root2), res);
    let root1 = tree!(0, tree!(3), tree!(1, None, tree!(2)));
    let root2 = tree!(0, tree!(3, tree!(2), None), tree!(1));
    let res = false;
    assert_eq!(Solution::flip_equiv(root1, root2), res);
}

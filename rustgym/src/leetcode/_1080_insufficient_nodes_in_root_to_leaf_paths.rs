struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self, limit: i32) -> TreeLink;
}

impl Postorder for TreeLink {
    fn postorder(self, limit: i32) -> TreeLink {
        if let Some(node) = self {
            let val = node.borrow_mut().val;
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if left.is_none() && right.is_none() {
                if val >= limit {
                    Some(node)
                } else {
                    None
                }
            } else {
                let l = left.postorder(limit - val);
                let r = right.postorder(limit - val);
                if l.is_some() || r.is_some() {
                    node.borrow_mut().left = l;
                    node.borrow_mut().right = r;
                    Some(node)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn sufficient_subset(root: TreeLink, limit: i32) -> TreeLink {
        root.postorder(limit)
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(
            2,
            tree!(4, tree!(8), tree!(9)),
            tree!(-99, tree!(-99), tree!(-99))
        ),
        tree!(
            3,
            tree!(-99, tree!(12), tree!(13)),
            tree!(7, tree!(-99), tree!(14))
        )
    );
    let limit = 1;
    let res = tree!(
        1,
        tree!(2, tree!(4, tree!(8), tree!(9)), None),
        tree!(3, None, tree!(7, None, tree!(14)))
    );
    assert_eq!(Solution::sufficient_subset(root, limit), res);
    let root = tree!(
        5,
        tree!(4, tree!(11, tree!(7), tree!(1)), None),
        tree!(8, tree!(17), tree!(4, tree!(5), tree!(3)))
    );
    let limit = 22;
    let res = tree!(
        5,
        tree!(4, tree!(11, tree!(7), None), None),
        tree!(8, tree!(17), tree!(4, tree!(5), None))
    );
    assert_eq!(Solution::sufficient_subset(root, limit), res);
    let root = tree!(1, tree!(2, tree!(-5), None), tree!(-3, tree!(4), None));
    let limit = -1;
    let res = tree!(1, None, tree!(-3, tree!(4), None));
    assert_eq!(Solution::sufficient_subset(root, limit), res);
}

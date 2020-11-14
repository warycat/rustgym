struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, p: i32, q: i32, lca: &mut TreeLink) -> (bool, bool);
}

impl Preorder for TreeLink {
    fn preorder(&self, p: i32, q: i32, lca: &mut TreeLink) -> (bool, bool) {
        if let Some(node) = self {
            let node = node.borrow();
            let l = node.left.preorder(p, q, lca);
            let r = node.right.preorder(p, q, lca);
            let res = (l.0 || r.0 || node.val == p, l.1 || r.1 || node.val == q);
            if lca.is_none() && res.0 && res.1 {
                *lca = tree!(node.val);
            }
            res
        } else {
            (false, false)
        }
    }
}

impl Solution {
    fn lowest_common_ancestor(root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        let p = p.as_ref().unwrap().borrow().val;
        let q = q.as_ref().unwrap().borrow().val;
        let mut res = None;
        root.preorder(p, q, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = tree!(5);
    let q = tree!(1);
    let res = tree!(3);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
    let root = tree!(
        3,
        tree!(5, tree!(6), tree!(2, tree!(7), tree!(4))),
        tree!(1, tree!(0), tree!(8))
    );
    let p = tree!(5);
    let q = tree!(4);
    let res = tree!(5);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
    let root = tree!(1, tree!(2), None);
    let p = tree!(1);
    let q = tree!(2);
    let res = tree!(1);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
}

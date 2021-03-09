struct Solution;
use rustgym_util::*;

impl Solution {
    fn lowest_common_ancestor(mut root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        while let Some(node) = root.clone() {
            let mut node = node.borrow_mut();
            let val = node.val;
            if val > p_val && val > q_val {
                root = node.left.take();
                continue;
            }
            if val < p_val && val < q_val {
                root = node.right.take();
                continue;
            }
            node.left.take();
            node.right.take();
            break;
        }
        root
    }
}

#[test]
fn test() {
    let root = tree!(
        6,
        tree!(2, tree!(0), tree!(4, tree!(3), tree!(5))),
        tree!(8, tree!(7), tree!(9))
    );
    let p = tree!(2);
    let q = tree!(8);
    let res = tree!(6);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
    let root = tree!(
        6,
        tree!(2, tree!(0), tree!(4, tree!(3), tree!(5))),
        tree!(8, tree!(7), tree!(9))
    );
    let p = tree!(2);
    let q = tree!(4);
    let res = tree!(2);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
    let root = tree!(2, tree!(1), None);
    let p = tree!(2);
    let q = tree!(1);
    let res = tree!(2);
    assert_eq!(Solution::lowest_common_ancestor(root, p, q), res);
}

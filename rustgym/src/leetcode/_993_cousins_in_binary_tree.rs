struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, depth: usize, parent: i32, res: &mut Option<(usize, i32)>, v: i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, depth: usize, parent: i32, res: &mut Option<(usize, i32)>, v: i32) {
        if res.is_some() {
            return;
        }
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if v == val {
                *res = Some((depth, parent));
            }
            node.left.preorder(depth + 1, val, res, v);
            node.right.preorder(depth + 1, val, res, v);
        }
    }
}

impl Solution {
    fn is_cousins(root: TreeLink, x: i32, y: i32) -> bool {
        let mut rx: Option<(usize, i32)> = None;
        let mut ry: Option<(usize, i32)> = None;
        root.preorder(0, 0, &mut rx, x);
        root.preorder(0, 0, &mut ry, y);
        if let (Some((dx, px)), Some((dy, py))) = (rx, ry) {
            dx == dy && px != py
        } else {
            false
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), None), tree!(3));
    let x = 4;
    let y = 3;
    let res = false;
    assert_eq!(Solution::is_cousins(root, x, y), res);
    let root = tree!(1, tree!(2, None, tree!(4)), tree!(3, None, tree!(5)));
    let x = 5;
    let y = 4;
    let res = true;
    assert_eq!(Solution::is_cousins(root, x, y), res);
    let root = tree!(1, tree!(2, None, tree!(4)), tree!(3));
    let x = 2;
    let y = 3;
    let res = false;
    assert_eq!(Solution::is_cousins(root, x, y), res);
}

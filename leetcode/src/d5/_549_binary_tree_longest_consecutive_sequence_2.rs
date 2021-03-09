struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, max: &mut usize) -> Option<(i32, usize, usize)>;
}

impl Postorder for TreeLink {
    fn postorder(&self, max: &mut usize) -> Option<(i32, usize, usize)> {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let mut inc = 1;
            let mut dec = 1;
            if let Some((lval, linc, ldec)) = node.left.postorder(max) {
                if lval + 1 == val {
                    inc = inc.max(linc + 1);
                }
                if lval - 1 == val {
                    dec = dec.max(ldec + 1);
                }
            }
            if let Some((rval, rinc, rdec)) = node.right.postorder(max) {
                if rval + 1 == val {
                    inc = inc.max(rinc + 1);
                }
                if rval - 1 == val {
                    dec = dec.max(rdec + 1);
                }
            }
            *max = (*max).max(inc + dec - 1);
            Some((val, inc, dec))
        } else {
            None
        }
    }
}

impl Solution {
    fn longest_consecutive(root: TreeLink) -> i32 {
        let mut res = 0;
        root.postorder(&mut res);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    let res = 2;
    assert_eq!(Solution::longest_consecutive(root), res);
    let root = tree!(2, tree!(1), tree!(3));
    let res = 3;
    assert_eq!(Solution::longest_consecutive(root), res);
}

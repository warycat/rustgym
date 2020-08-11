struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, path: u32, all: &mut i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, mut path: u32, all: &mut i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            path ^= 1 << val;
            if node.left.is_none() && node.right.is_none() {
                if path.count_ones() < 2 {
                    *all += 1;
                }
            }
            node.left.preorder(path, all);
            node.right.preorder(path, all);
        }
    }
}

impl Solution {
    fn pseudo_palindromic_paths(root: TreeLink) -> i32 {
        let mut res = 0;
        root.preorder(0, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(3, tree!(3), tree!(1)), tree!(1, None, tree!(1)));
    let res = 2;
    assert_eq!(Solution::pseudo_palindromic_paths(root), res);
    let root = tree!(2, tree!(1, tree!(1), tree!(3, None, tree!(1))), tree!(1));
    let res = 1;
    assert_eq!(Solution::pseudo_palindromic_paths(root), res);
}

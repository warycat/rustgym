struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, count: &mut usize, res: &mut i32, k: usize);
}

impl Inorder for TreeLink {
    fn inorder(&self, count: &mut usize, res: &mut i32, k: usize) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            left.inorder(count, res, k);
            *count += 1;
            if *count == k {
                *res = node.val;
            }
            right.inorder(count, res, k);
        }
    }
}

impl Solution {
    fn kth_smallest(root: TreeLink, k: i32) -> i32 {
        let mut count = 0;
        let mut res = 0;
        root.inorder(&mut count, &mut res, k as usize);
        res
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(1, None, tree!(2)), tree!(4));
    let k = 1;
    let res = 1;
    assert_eq!(Solution::kth_smallest(root, k), res);
    let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
    let k = 3;
    let res = 3;
    assert_eq!(Solution::kth_smallest(root, k), res);
}

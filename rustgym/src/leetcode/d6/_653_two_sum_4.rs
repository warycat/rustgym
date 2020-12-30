struct Solution;
use rustgym_util::*;
use std::cmp::Ordering::*;

trait Inorder {
    fn inorder(&self, v: &mut Vec<i32>, target: i32);
}

impl Inorder for TreeLink {
    fn inorder(&self, v: &mut Vec<i32>, target: i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            Self::inorder(left, v, target);
            v.push(node.val);
            Self::inorder(right, v, target);
        }
    }
}

impl Solution {
    fn find_target(root: TreeLink, k: i32) -> bool {
        let mut v = vec![];
        root.inorder(&mut v, k);
        let n = v.len();
        let mut l = 0;
        let mut r = n - 1;
        while l < r {
            let sum = v[l] + v[r];
            match sum.cmp(&k) {
                Greater => {
                    r -= 1;
                }
                Less => {
                    l += 1;
                }
                Equal => {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
    assert_eq!(Solution::find_target(root, 9), true);
}

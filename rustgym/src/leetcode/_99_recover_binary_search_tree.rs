struct Solution;
use rustgym_util::*;
use std::mem::swap;

trait Inorder {
    fn inorder(&self, prev: &mut TreeLink, first: &mut TreeLink, second: &mut TreeLink);
}

impl Inorder for TreeLink {
    fn inorder(&self, prev: &mut TreeLink, first: &mut TreeLink, second: &mut TreeLink) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.inorder(prev, first, second);
            if let Some(prev_val) = prev.clone() {
                if prev_val.borrow().val >= node.val {
                    if first.is_none() {
                        *first = prev.clone();
                    }
                    *second = self.clone();
                }
            }
            *prev = self.clone();
            node.right.inorder(prev, first, second);
        }
    }
}

impl Solution {
    fn recover_tree(root: &mut TreeLink) {
        let mut prev = None;
        let mut first = None;
        let mut second = None;
        root.inorder(&mut prev, &mut first, &mut second);
        swap(
            &mut first.unwrap().borrow_mut().val,
            &mut second.unwrap().borrow_mut().val,
        )
    }
}

#[test]
fn test() {
    let mut root = tree!(1, tree!(3, None, tree!(2)), None);
    Solution::recover_tree(&mut root);
    let res = tree!(3, tree!(1, None, tree!(2)), None);
    assert_eq!(root, res);
    let mut root = tree!(3, tree!(1), tree!(4, tree!(2), None));
    Solution::recover_tree(&mut root);
    let res = tree!(2, tree!(1), tree!(4, tree!(3), None));
    assert_eq!(root, res);
}

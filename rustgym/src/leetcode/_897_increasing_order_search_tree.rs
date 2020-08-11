struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(self, next: TreeLink) -> TreeLink;
}

impl Inorder for TreeLink {
    fn inorder(self, next: TreeLink) -> TreeLink {
        if let Some(node) = self.as_ref() {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let res = Self::inorder(left, self.clone());
            node.right = Self::inorder(right, next);
            res
        } else {
            next
        }
    }
}

impl Solution {
    fn increasing_bst(root: TreeLink) -> TreeLink {
        root.inorder(None)
    }
}

#[test]
fn test() {
    let root = tree!(
        5,
        tree!(3, tree!(2, tree!(1), None), tree!(4)),
        tree!(6, None, tree!(8, tree!(7), tree!(9)))
    );
    let res = tree!(
        1,
        None,
        tree!(
            2,
            None,
            tree!(
                3,
                None,
                tree!(
                    4,
                    None,
                    tree!(
                        5,
                        None,
                        tree!(6, None, tree!(7, None, tree!(8, None, tree!(9))))
                    )
                )
            )
        )
    );
    assert_eq!(Solution::increasing_bst(root), res);
}

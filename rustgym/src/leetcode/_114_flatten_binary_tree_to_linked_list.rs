struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(self, prev: &mut TreeLink);
}

impl Postorder for TreeLink {
    fn postorder(self, prev: &mut TreeLink) {
        if let Some(node) = self {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            right.postorder(prev);
            left.postorder(prev);
            node.borrow_mut().right = prev.take();
            *prev = Some(node);
        }
    }
}

impl Solution {
    fn flatten(root: &mut TreeLink) {
        let mut prev: TreeLink = None;
        root.take().postorder(&mut prev);
        *root = prev;
    }
}

#[test]
fn test() {
    let mut root = tree!(1, tree!(2, tree!(3), tree!(4)), tree!(5, None, tree!(6)));
    let res = tree!(
        1,
        None,
        tree!(
            2,
            None,
            tree!(3, None, tree!(4, None, tree!(5, None, tree!(6))))
        )
    );
    Solution::flatten(&mut root);
    assert_eq!(root, res);
}

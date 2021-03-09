struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, level: usize, view: &mut Vec<i32>);
}

impl Preorder for TreeLink {
    fn preorder(&self, level: usize, view: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if level == view.len() {
                view.push(val);
            } else {
                view[level] = val;
            }
            node.left.preorder(level + 1, view);
            node.right.preorder(level + 1, view);
        }
    }
}

impl Solution {
    fn right_side_view(root: TreeLink) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        root.preorder(0, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(5)), tree!(3, None, tree!(4)));
    let res = vec![1, 3, 4];
    assert_eq!(Solution::right_side_view(root), res);
}

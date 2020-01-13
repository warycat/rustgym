struct Solution;
use crate::util::*;

impl Solution {
    fn right_side_view(root: TreeLink) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        Self::dfs(&root, 0, &mut res);
        res
    }

    fn dfs(link: &TreeLink, level: usize, view: &mut Vec<i32>) {
        if let Some(node) = link {
            let node = node.borrow();
            let val = node.val;
            if view.len() <= level {
                view.push(val);
            } else {
                view[level] = val;
            }
            Self::dfs(&node.left, level + 1, view);
            Self::dfs(&node.right, level + 1, view);
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(5)), tree!(3, None, tree!(4)));
    let res = vec![1, 3, 4];
    assert_eq!(Solution::right_side_view(root), res);
}

struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, path: &mut Vec<i32>, res: &mut bool, arr: &[i32]);
}

impl Preorder for TreeLink {
    fn preorder(&self, path: &mut Vec<i32>, res: &mut bool, arr: &[i32]) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            path.push(val);
            if node.left.is_none() && node.right.is_none() && *path == arr {
                *res = true;
            }
            node.left.preorder(path, res, arr);
            node.right.preorder(path, res, arr);
            path.pop();
        }
    }
}

impl Solution {
    fn is_valid_sequence(root: TreeLink, arr: Vec<i32>) -> bool {
        let mut path = vec![];
        let mut res = false;
        root.preorder(&mut path, &mut res, &arr);
        res
    }
}

#[test]
fn test() {
    let root = tree!(
        0,
        tree!(1, tree!(0, None, tree!(1)), tree!(1, tree!(0), tree!(0))),
        tree!(0, tree!(0), None)
    );
    let arr = vec![0, 1, 0, 1];
    let res = true;
    assert_eq!(Solution::is_valid_sequence(root, arr), res);
    let root = tree!(
        0,
        tree!(1, tree!(0, None, tree!(1)), tree!(1, tree!(0), tree!(0))),
        tree!(0, tree!(0), None)
    );
    let arr = vec![0, 0, 1];
    let res = false;
    assert_eq!(Solution::is_valid_sequence(root, arr), res);
    let root = tree!(
        0,
        tree!(1, tree!(0, None, tree!(1)), tree!(1, tree!(0), tree!(0))),
        tree!(0, tree!(0), None)
    );
    let arr = vec![0, 1, 1];
    let res = false;
    assert_eq!(Solution::is_valid_sequence(root, arr), res);
}

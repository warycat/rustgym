struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, row: usize, max: &mut Vec<i32>);
}

impl Preorder for TreeLink {
    fn preorder(&self, row: usize, max: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            if row == max.len() {
                max.push(node.val);
            } else {
                max[row] = max[row].max(node.val);
            }
            node.left.preorder(row + 1, max);
            node.right.preorder(row + 1, max);
        }
    }
}

impl Solution {
    fn largest_values(root: TreeLink) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        root.preorder(0, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(3, tree!(5), tree!(3)), tree!(2, None, tree!(9)));
    let res = vec![1, 3, 9];
    assert_eq!(Solution::largest_values(root), res);
}

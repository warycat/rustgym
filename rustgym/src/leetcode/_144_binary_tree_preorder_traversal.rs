struct Solution;

trait Preorder {
    fn preorder(&self, all: &mut Vec<i32>);
}

impl Preorder for TreeLink {
    fn preorder(&self, all: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            all.push(node.val);
            node.left.preorder(all);
            node.right.preorder(all);
        }
    }
}

impl Solution {
    fn preorder_traversal(root: TreeLink) -> Vec<i32> {
        let mut res = vec![];
        root.preorder(&mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    let res = vec![1, 2, 3];
    assert_eq!(Solution::preorder_traversal(root), res);
}

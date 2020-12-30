struct Solution;
use rustgym_util::*;

impl Solution {
    fn inorder_traversal(root: TreeLink) -> Vec<i32> {
        let mut cur = root;
        let mut stack: Vec<TreeLink> = vec![];
        let mut res = vec![];
        while cur.is_some() || !stack.is_empty() {
            while let Some(node) = cur {
                let left = node.borrow_mut().left.take();
                stack.push(Some(node));
                cur = left;
            }
            let node = stack.pop().unwrap().unwrap();
            res.push(node.borrow().val);
            cur = node.borrow_mut().right.take();
        }
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    let res = vec![1, 3, 2];
    assert_eq!(Solution::inorder_traversal(root), res);
}

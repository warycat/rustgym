struct Solution;
use rustgym_util::*;
use std::cmp::Ordering::*;

trait Delete {
    fn delete(self, key: i32) -> TreeLink;
    fn smallest(&self) -> i32;
}

impl Delete for TreeLink {
    fn delete(self, key: i32) -> TreeLink {
        if let Some(link) = self {
            let mut node = link.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            let val = node.val;
            match key.cmp(&val) {
                Equal => match (left, right) {
                    (None, None) => None,
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),
                    (left, right) => {
                        let smallest = right.smallest();
                        tree!(smallest, left, right.delete(smallest))
                    }
                },
                Less => tree!(val, left.delete(key), right),
                Greater => tree!(val, left, right.delete(key)),
            }
        } else {
            None
        }
    }
    fn smallest(&self) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if node.left.is_some() {
                node.left.smallest()
            } else {
                val
            }
        } else {
            unreachable!()
        }
    }
}

impl Solution {
    fn delete_node(root: TreeLink, key: i32) -> TreeLink {
        root.delete(key)
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(3, tree!(2), tree!(4)), tree!(6, None, tree!(7)));
    let key = 3;
    let res = tree!(5, tree!(4, tree!(2), None), tree!(6, None, tree!(7)));
    assert_eq!(Solution::delete_node(root, key), res);
}

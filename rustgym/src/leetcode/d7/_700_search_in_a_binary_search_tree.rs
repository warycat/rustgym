struct Solution;
use rustgym_util::*;

trait Search {
    fn find(&self, val: i32) -> TreeLink;
}

impl Search for TreeLink {
    fn find(&self, val: i32) -> TreeLink {
        if let Some(node) = self {
            let temp = node.clone();
            let node = node.borrow();
            if val == node.val {
                Some(temp)
            } else {
                if val < node.val {
                    Self::find(&node.left, val)
                } else {
                    Self::find(&node.right, val)
                }
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn search_bst(root: TreeLink, val: i32) -> TreeLink {
        root.find(val)
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(3));
    let res = tree!(2, tree!(1), tree!(3));
    assert_eq!(Solution::search_bst(root, 2), res);
}

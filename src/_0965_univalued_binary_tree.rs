struct Solution;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn branch(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> Link {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

struct Tree {
    root: Link,
}

impl Tree {
    fn new(root: Link) -> Self {
        Tree { root }
    }
    fn is_unival(&self) -> bool {
        let mut val: Option<i32> = None;
        Tree::preorder(&self.root, &mut val)
    }
    fn preorder(link: &Link, val: &mut Option<i32>) -> bool {
        if let Some(node) = link {
            let node = node.borrow();
            let node_val = node.val;
            if let Some(val) = val {
                if *val != node_val {
                    return false;
                }
            } else {
                *val = Some(node_val);
            }
            Tree::preorder(&node.left, val) && Tree::preorder(&node.right, val)
        } else {
            true
        }
    }
}

impl Solution {
    fn is_unival_tree(root: Link) -> bool {
        let tree = Tree::new(root);
        tree.is_unival()
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(1, TreeNode::leaf(1), TreeNode::leaf(1)),
        TreeNode::branch(1, None, TreeNode::leaf(1)),
    );
    assert_eq!(Solution::is_unival_tree(root), true);
    let root = TreeNode::branch(
        2,
        TreeNode::branch(2, TreeNode::leaf(5), TreeNode::leaf(2)),
        TreeNode::leaf(2),
    );
    assert_eq!(Solution::is_unival_tree(root), false);
}

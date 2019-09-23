struct Solution;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

use std::cell::RefCell;
use std::i32;
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

    fn diameter(&self) -> i32 {
        let mut max: i32 = 0;
        Tree::max_depth(&self.root, &mut max);
        max
    }

    fn max_depth(link: &Link, max: &mut i32) -> i32 {
        if let Some(node) = link {
            let left = Tree::max_depth(&node.borrow().left, max);
            let right = Tree::max_depth(&node.borrow().right, max);
            *max = i32::max(*max, left + right);
            i32::max(left + 1, right + 1)
        } else {
            0
        }
    }
}

impl Solution {
    fn diameter_of_binary_tree(root: Link) -> i32 {
        let tree = Tree::new(root);
        tree.diameter()
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(2, TreeNode::leaf(4), TreeNode::leaf(5)),
        TreeNode::leaf(3),
    );
    assert_eq!(Solution::diameter_of_binary_tree(root), 3);
}

struct Solution;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Solution {
    fn convert_bst(mut root: Link) -> Link {
        let mut sum = 0;
        Solution::inorder(&mut root, &mut sum);
        root
    }

    fn inorder(link: &mut Link, sum: &mut i32) {
        if let Some(node) = link {
            let mut node = node.borrow_mut();
            Solution::inorder(&mut node.right, sum);
            *sum += node.val;
            node.val = *sum;
            Solution::inorder(&mut node.left, sum);
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(5, TreeNode::leaf(2), TreeNode::leaf(13));
    let greater = TreeNode::branch(18, TreeNode::leaf(20), TreeNode::leaf(13));
    assert_eq!(Solution::convert_bst(root), greater);
}

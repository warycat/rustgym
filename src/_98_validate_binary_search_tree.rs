struct Solution;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    fn node(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> Link {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
    fn inorder(link: &Link, visit: &mut dyn FnMut(i32)) {
        if let Some(node) = link {
            let node = node.borrow();
            Self::inorder(&node.left, visit);
            visit(node.val);
            Self::inorder(&node.right, visit);
        }
    }
}

impl Solution {
    fn is_valid_bst(root: Link) -> bool {
        let mut prev: Option<i32> = None;
        let mut res = true;
        TreeNode::inorder(&root, &mut |x| {
            if let Some(y) = prev {
                if x <= y {
                    res = false;
                }
            }
            prev = Some(x);
        });
        res
    }
}

#[test]
fn test() {
    let root = TreeNode::node(2, TreeNode::leaf(1), TreeNode::leaf(3));
    assert_eq!(Solution::is_valid_bst(root), true);
    let root = TreeNode::node(
        5,
        TreeNode::leaf(1),
        TreeNode::node(4, TreeNode::leaf(3), TreeNode::leaf(6)),
    );
    assert_eq!(Solution::is_valid_bst(root), false);
}

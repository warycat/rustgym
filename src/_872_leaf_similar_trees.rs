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
    fn dfs(link: &Link, leaves: &mut Vec<i32>) {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if left.is_none() && right.is_none() {
                leaves.push(node.val);
            } else {
                Self::dfs(left, leaves);
                Self::dfs(right, leaves);
            }
        }
    }
}

impl Solution {
    fn leaf_similar(root1: Link, root2: Link) -> bool {
        let mut leaves1: Vec<i32> = vec![];
        let mut leaves2: Vec<i32> = vec![];
        TreeNode::dfs(&root1, &mut leaves1);
        TreeNode::dfs(&root2, &mut leaves2);
        leaves1 == leaves2
    }
}

#[test]
fn test() {
    let root1 = TreeNode::branch(
        3,
        TreeNode::branch(
            5,
            TreeNode::leaf(6),
            TreeNode::branch(2, TreeNode::leaf(7), TreeNode::leaf(4)),
        ),
        TreeNode::branch(1, TreeNode::leaf(9), TreeNode::leaf(8)),
    );
    let root2 = TreeNode::branch(
        4,
        TreeNode::branch(
            5,
            TreeNode::leaf(6),
            TreeNode::branch(2, TreeNode::leaf(7), TreeNode::leaf(4)),
        ),
        TreeNode::branch(1, TreeNode::leaf(9), TreeNode::leaf(8)),
    );
    assert_eq!(Solution::leaf_similar(root1, root2), true);
}

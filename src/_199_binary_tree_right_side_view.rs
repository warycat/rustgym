struct Solution;
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

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

use std::cell::RefCell;
use std::rc::Rc;
type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn right_side_view(root: Link) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        Self::dfs(&root, 0, &mut res);
        res
    }

    fn dfs(link: &Link, level: usize, view: &mut Vec<i32>) {
        if let Some(node) = link {
            let node = node.borrow();
            let val = node.val;
            if view.len() <= level {
                view.push(val);
            } else {
                view[level] = val;
            }
            Self::dfs(&node.left, level + 1, view);
            Self::dfs(&node.right, level + 1, view);
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(2, None, TreeNode::leaf(5)),
        TreeNode::branch(3, None, TreeNode::leaf(4)),
    );
    let res = vec![1, 3, 4];
    assert_eq!(Solution::right_side_view(root), res);
}

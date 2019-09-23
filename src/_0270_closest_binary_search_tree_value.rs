struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

use std::cell::RefCell;
use std::f64;
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

    fn search(&self, target: f64) -> i32 {
        let mut diff = f64::MAX;
        let mut res = 0;
        Self::preorder(&self.root, &mut diff, &mut res, target);
        res
    }

    fn preorder(link: &Link, diff: &mut f64, res: &mut i32, target: f64) {
        if let Some(node) = link {
            let node = node.borrow();
            let val = node.val as f64;
            let delta = (val - target).abs();
            if delta < *diff {
                *diff = delta;
                *res = node.val;
            }
            if target < val {
                Self::preorder(&node.left, diff, res, target);
            }
            if target > val {
                Self::preorder(&node.right, diff, res, target)
            }
        }
    }
}

impl Solution {
    fn closest_value(root: Link, target: f64) -> i32 {
        let tree = Tree::new(root);
        tree.search(target)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        4,
        TreeNode::branch(2, TreeNode::leaf(1), TreeNode::leaf(3)),
        TreeNode::leaf(5),
    );
    assert_eq!(Solution::closest_value(root, 3.714_286), 4);
}

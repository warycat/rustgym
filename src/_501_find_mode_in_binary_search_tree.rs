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

    fn inorder(
        link: &Link,
        prev: &mut Option<i32>,
        count: &mut usize,
        f: &mut impl FnMut(i32, usize),
    ) {
        if let Some(node) = link.as_ref() {
            let node = node.borrow();
            Self::inorder(&node.left, prev, count, f);
            if let Some(prev_val) = prev.as_mut() {
                if *prev_val == node.val {
                    *count += 1;
                } else {
                    *count = 1;
                    *prev = Some(node.val);
                }
            } else {
                *prev = Some(node.val);
                *count = 1;
            }
            f(node.val, *count);
            Self::inorder(&node.right, prev, count, f);
        }
    }
}

impl Solution {
    fn find_mode(root: Link) -> Vec<i32> {
        let mut max = 0;
        let mut count = 0;
        let mut prev: Option<i32> = None;
        let mut modes: Vec<i32> = vec![];
        TreeNode::inorder(&root, &mut prev, &mut count, &mut |_, count| {
            max = usize::max(count, max);
        });
        prev = None;
        count = 0;
        TreeNode::inorder(&root, &mut prev, &mut count, &mut |val, count| {
            if count == max {
                modes.push(val);
            }
        });
        modes
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(1, None, TreeNode::branch(2, TreeNode::leaf(2), None));
    assert_eq!(Solution::find_mode(root), vec![2]);
}

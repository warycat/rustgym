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
    fn range_sum_bst(root: Link, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        Solution::dfs(&root, l, r, &mut sum);
        sum
    }
    fn dfs(link: &Link, l: i32, r: i32, sum: &mut i32) {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val;
            if val >= l && val <= r {
                *sum += val;
            }
            if val > l {
                Solution::dfs(left, l, r, sum)
            }
            if val < r {
                Solution::dfs(right, l, r, sum);
            }
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        10,
        TreeNode::branch(5, TreeNode::leaf(3), TreeNode::leaf(7)),
        TreeNode::branch(15, None, TreeNode::leaf(18)),
    );
    assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
}

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
    fn dfs(link: &Link, level: usize, levels: &mut Vec<Vec<i32>>) {
        if let Some(node) = link {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if levels.len() == level {
                levels.push(vec![node.val]);
            } else {
                levels[level].push(node.val);
            }
            Self::dfs(left, level + 1, levels);
            Self::dfs(right, level + 1, levels);
        }
    }
}

impl Solution {
    fn average_of_levels(root: Link) -> Vec<f64> {
        let mut levels: Vec<Vec<i32>> = vec![];
        TreeNode::dfs(&root, 0, &mut levels);
        levels
            .iter()
            .map(|v| v.iter().map(|&x| x as f64).sum::<f64>() as f64 / v.len() as f64)
            .collect()
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    let res: Vec<f64> = vec![3f64, 14.5, 11f64];
    assert_eq!(Solution::average_of_levels(root), res);
}

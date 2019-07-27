struct Solution;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

impl TreeNode {
    fn branch(val: i32, left: Tree, right: Tree) -> Tree {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
    fn leaf(val: i32) -> Tree {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn path_sum_r(root: &Tree, prefix: i32, sum: i32, prefix_map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val;
            let mut res = 0i32;
            let prefix = prefix + val;
            let count = *prefix_map.get(&(prefix - sum)).unwrap_or(&0);
            res += count;
            *prefix_map.entry(prefix).or_default() += 1;
            res += Solution::path_sum_r(left, prefix, sum, prefix_map);
            res += Solution::path_sum_r(right, prefix, sum, prefix_map);
            *prefix_map.entry(prefix).or_default() -= 1;
            res
        } else {
            0
        }
    }
    fn path_sum(root: Tree, sum: i32) -> i32 {
        let prefix_map: &mut HashMap<i32, i32> = &mut HashMap::new();
        prefix_map.insert(0, 1);
        Solution::path_sum_r(&root, 0, sum, prefix_map)
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        10,
        TreeNode::branch(
            5,
            TreeNode::branch(3, TreeNode::leaf(3), TreeNode::leaf(-2)),
            TreeNode::branch(2, None, TreeNode::leaf(1)),
        ),
        TreeNode::branch(-3, None, TreeNode::leaf(11)),
    );
    assert_eq!(Solution::path_sum(root, 8), 3);
}

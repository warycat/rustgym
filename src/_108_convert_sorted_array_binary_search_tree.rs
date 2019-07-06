struct Solution {}

#[derive(PartialEq, Eq, Debug)]
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
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn sorted_array_to_bst(nums: Vec<i32>) -> Tree {
        let n = nums.len();
        match n {
            0 => None,
            1 => TreeNode::leaf(nums[0]),
            _ => {
                let mid = n / 2;
                let left = nums[..mid].to_vec();
                let right = nums[mid + 1..].to_vec();
                TreeNode::branch(
                    nums[mid],
                    Solution::sorted_array_to_bst(left),
                    Solution::sorted_array_to_bst(right),
                )
            }
        }
    }
}

#[test]
fn test() {
    let nums = vec![-10, -3, 0, 5, 9];
    let bst = TreeNode::branch(
        0,
        TreeNode::branch(-3, TreeNode::leaf(-10), None),
        TreeNode::branch(9, TreeNode::leaf(5), None),
    );
    assert_eq!(Solution::sorted_array_to_bst(nums), bst);
}

struct Solution;

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
use std::collections::VecDeque;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

struct Pair {
    tree: Tree,
    level: usize,
}

impl Solution {
    fn level_order_bottom(root: Tree) -> Vec<Vec<i32>> {
        let mut levels: Vec<Vec<i32>> = vec![];
        let mut queue: VecDeque<Pair> = VecDeque::new();
        if root.is_some() {
            queue.push_back(Pair {
                tree: root,
                level: 0,
            });
            while let Some(pair) = queue.pop_front() {
                let node = pair.tree.unwrap();
                let mut node = node.borrow_mut();
                let level = pair.level;
                let val = node.val;
                let left = node.left.take();
                let right = node.right.take();
                if let Some(group) = levels.get_mut(level) {
                    group.push(val);
                } else {
                    let group = vec![val];
                    levels.insert(level, group);
                }
                if left.is_some() {
                    queue.push_back(Pair {
                        tree: left,
                        level: level + 1,
                    });
                }
                if right.is_some() {
                    queue.push_back(Pair {
                        tree: right,
                        level: level + 1,
                    });
                }
            }
        }
        levels.reverse();
        levels
    }
}

#[test]
fn test() {
    let tree = TreeNode::branch(
        3,
        TreeNode::leaf(9),
        TreeNode::branch(20, TreeNode::leaf(15), TreeNode::leaf(7)),
    );
    let nodes = vec![vec![15, 7], vec![9, 20], vec![3]];
    assert_eq!(Solution::level_order_bottom(tree), nodes);
}

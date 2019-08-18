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

struct Path {
    stack: Vec<i32>,
}

impl ToString for Path {
    fn to_string(&self) -> String {
        let s: Vec<String> = self.stack.iter().map(|x| x.to_string()).collect();
        s.join("->")
    }
}

impl Solution {
    fn binary_tree_paths(root: Link) -> Vec<String> {
        let mut path = Path { stack: vec![] };
        let mut res = vec![];
        Solution::dfs(&root, &mut path, &mut res);
        res
    }

    fn dfs(link: &Link, path: &mut Path, v: &mut Vec<String>) {
        if let Some(node) = link {
            let node = node.borrow();
            path.stack.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                v.push(path.to_string());
            }
            if node.left.is_some() {
                Solution::dfs(&node.left, path, v);
            }
            if node.right.is_some() {
                Solution::dfs(&node.right, path, v);
            }
            path.stack.pop();
        }
    }
}

#[test]
fn test() {
    let root = TreeNode::branch(
        1,
        TreeNode::branch(2, None, TreeNode::leaf(5)),
        TreeNode::leaf(3),
    );
    let paths: Vec<String> = ["1->2->5", "1->3"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::binary_tree_paths(root), paths);
}

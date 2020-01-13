struct Solution;
use crate::util::*;

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
    fn binary_tree_paths(root: TreeLink) -> Vec<String> {
        let mut path = Path { stack: vec![] };
        let mut res = vec![];
        Solution::dfs(&root, &mut path, &mut res);
        res
    }

    fn dfs(link: &TreeLink, path: &mut Path, v: &mut Vec<String>) {
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
    let root = tree!(1, tree!(2, None, tree!(5)), tree!(3));
    let paths: Vec<String> = ["1->2->5", "1->3"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::binary_tree_paths(root), paths);
}

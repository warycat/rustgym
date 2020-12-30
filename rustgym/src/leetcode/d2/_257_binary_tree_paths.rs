struct Solution;
use rustgym_util::*;

struct Path {
    stack: Vec<i32>,
}

impl ToString for Path {
    fn to_string(&self) -> String {
        let s: Vec<String> = self.stack.iter().map(|x| x.to_string()).collect();
        s.join("->")
    }
}

trait Preorder {
    fn preorder(&self, path: &mut Path, v: &mut Vec<String>);
}

impl Preorder for TreeLink {
    fn preorder(&self, path: &mut Path, v: &mut Vec<String>) {
        if let Some(node) = self {
            let node = node.borrow();
            path.stack.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                v.push(path.to_string());
            }
            if node.left.is_some() {
                node.left.preorder(path, v);
            }
            if node.right.is_some() {
                node.right.preorder(path, v);
            }
            path.stack.pop();
        }
    }
}

impl Solution {
    fn binary_tree_paths(root: TreeLink) -> Vec<String> {
        let mut path = Path { stack: vec![] };
        let mut res = vec![];
        root.preorder(&mut path, &mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, None, tree!(5)), tree!(3));
    let paths: Vec<String> = vec_string!["1->2->5", "1->3"];
    assert_eq!(Solution::binary_tree_paths(root), paths);
}

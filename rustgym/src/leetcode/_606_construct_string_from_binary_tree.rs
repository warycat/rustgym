struct Solution;
use rustgym_util::*;

trait TreeToString {
    fn tree_to_string(&self) -> String;
}

impl TreeToString for TreeLink {
    fn tree_to_string(&self) -> String {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            match (&left, &right) {
                (Some(_), Some(_)) => format!(
                    "{}({})({})",
                    node.val,
                    left.tree_to_string(),
                    right.tree_to_string(),
                ),
                (Some(_), None) => format!("{}({})", node.val, left.tree_to_string()),
                (None, Some(_)) => format!("{}()({})", node.val, right.tree_to_string()),
                (None, None) => format!("{}", node.val),
            }
        } else {
            "".to_string()
        }
    }
}

impl Solution {
    fn tree2str(t: TreeLink) -> String {
        t.tree_to_string()
    }
}

#[test]
fn test() {
    let t = tree!(1, tree!(2, tree!(4), None), tree!(3));
    assert_eq!(Solution::tree2str(t), "1(2(4))(3)".to_string());
}

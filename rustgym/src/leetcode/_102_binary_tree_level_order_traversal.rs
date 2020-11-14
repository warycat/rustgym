struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, levels: &mut Vec<Vec<i32>>, level: usize);
}

impl Preorder for TreeLink {
    fn preorder(&self, levels: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(node) = self {
            let val = node.borrow().val;
            if levels.len() == level {
                levels.push(vec![val]);
            } else {
                levels[level].push(val);
            }
            node.borrow().left.preorder(levels, level + 1);
            node.borrow().right.preorder(levels, level + 1);
        }
    }
}

impl Solution {
    fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut res = vec![];
        root.preorder(&mut res, 0);
        res
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<Vec<i32>> = vec_vec_i32![[3], [9, 20], [15, 7]];
    assert_eq!(Solution::level_order(root), res);
}

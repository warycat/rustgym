struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, level: usize, levels: &mut Vec<Vec<i32>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, level: usize, levels: &mut Vec<Vec<i32>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if level == levels.len() {
                levels.push(vec![val]);
            } else {
                levels[level].push(val);
            }
            node.left.preorder(level + 1, levels);
            node.right.preorder(level + 1, levels);
        }
    }
}

impl Solution {
    fn zigzag_level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut levels: Vec<Vec<i32>> = vec![];
        root.preorder(0, &mut levels);
        for i in 0..levels.len() {
            if i % 2 == 1 {
                levels[i].reverse();
            }
        }
        levels
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = vec_vec_i32![[3], [20, 9], [15, 7]];
    assert_eq!(Solution::zigzag_level_order(root), res);
}

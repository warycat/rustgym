struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, level: usize, levels: &mut Vec<Vec<i32>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, level: usize, levels: &mut Vec<Vec<i32>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if levels.len() == level {
                levels.push(vec![node.val]);
            } else {
                levels[level].push(node.val);
            }
            left.preorder(level + 1, levels);
            right.preorder(level + 1, levels);
        }
    }
}

impl Solution {
    fn average_of_levels(root: TreeLink) -> Vec<f64> {
        let mut levels: Vec<Vec<i32>> = vec![];
        root.preorder(0, &mut levels);
        levels
            .iter()
            .map(|v| v.iter().map(|&x| x as f64).sum::<f64>() as f64 / v.len() as f64)
            .collect()
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<f64> = vec![3f64, 14.5, 11f64];
    assert_eq!(Solution::average_of_levels(root), res);
}

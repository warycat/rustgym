struct Solution;
use rustgym_util::*;
use std::collections::HashMap;

trait Preorder {
    fn preorder(
        &self,
        row: usize,
        pos: u32,
        min: &mut HashMap<usize, u32>,
        max: &mut HashMap<usize, u32>,
        diff: &mut u32,
    );
}

impl Preorder for TreeLink {
    fn preorder(
        &self,
        row: usize,
        pos: u32,
        min: &mut HashMap<usize, u32>,
        max: &mut HashMap<usize, u32>,
        diff: &mut u32,
    ) {
        if let Some(node) = self {
            min.entry(row).or_insert(pos);
            max.entry(row).or_insert(pos);
            *min.get_mut(&row).unwrap() = min[&row].min(pos);
            *max.get_mut(&row).unwrap() = max[&row].max(pos);
            *diff = (*diff).max(max[&row] - min[&row] + 1);
            let node = node.borrow();
            node.left.preorder(row + 1, pos << 1, min, max, diff);
            node.right.preorder(row + 1, (pos << 1) + 1, min, max, diff);
        }
    }
}

impl Solution {
    fn width_of_binary_tree(root: TreeLink) -> i32 {
        let mut min: HashMap<usize, u32> = HashMap::new();
        let mut max: HashMap<usize, u32> = HashMap::new();
        let mut res = 0;
        root.preorder(0, 0, &mut min, &mut max, &mut res);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(3, tree!(5), tree!(3)), tree!(2, None, tree!(9)));
    let res = 4;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(1, tree!(3, tree!(5), tree!(3)), None);
    let res = 2;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(1, tree!(3, tree!(5), None), tree!(2));
    let res = 2;
    assert_eq!(Solution::width_of_binary_tree(root), res);
    let root = tree!(
        1,
        tree!(3, tree!(5, tree!(6), None), None),
        tree!(2, None, tree!(9, None, tree!(7)))
    );
    let res = 8;
    assert_eq!(Solution::width_of_binary_tree(root), res);
}

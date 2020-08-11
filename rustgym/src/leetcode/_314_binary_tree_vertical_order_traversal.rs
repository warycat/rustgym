struct Solution;
use rustgym_util::*;
use std::collections::BTreeMap;

trait Preorder {
    fn preorder(&self, row: i32, col: i32, grid: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>);
}

impl Preorder for TreeLink {
    fn preorder(&self, row: i32, col: i32, grid: &mut BTreeMap<i32, BTreeMap<i32, Vec<i32>>>) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            grid.entry(col)
                .or_default()
                .entry(row)
                .or_default()
                .push(val);

            left.preorder(row + 1, col - 1, grid);
            right.preorder(row + 1, col + 1, grid);
        }
    }
}

impl Solution {
    fn vertical_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut grid: BTreeMap<i32, BTreeMap<i32, Vec<i32>>> = BTreeMap::new();
        root.preorder(0, 0, &mut grid);
        grid.into_iter()
            .map(|(_, row)| row.into_iter().flat_map(|(_, v)| v.into_iter()).collect())
            .collect()
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res = vec_vec_i32![[9], [3, 15], [20], [7]];
    assert_eq!(Solution::vertical_order(root), res);
    let root = tree!(
        3,
        tree!(9, tree!(4), tree!(0)),
        tree!(8, tree!(1), tree!(7))
    );
    let res = vec_vec_i32![[4], [9], [3, 0, 1], [8], [7]];
    assert_eq!(Solution::vertical_order(root), res);
}

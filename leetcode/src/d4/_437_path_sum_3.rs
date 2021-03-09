struct Solution;
use rustgym_util::*;
use std::collections::HashMap;

trait PathSum {
    fn path_sum(&self, prefix: i32, sum: i32, prefix_map: &mut HashMap<i32, i32>) -> i32;
}

impl PathSum for TreeLink {
    fn path_sum(&self, prefix: i32, sum: i32, prefix_map: &mut HashMap<i32, i32>) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val;
            let mut res = 0i32;
            let prefix = prefix + val;
            let count = *prefix_map.get(&(prefix - sum)).unwrap_or(&0);
            res += count;
            *prefix_map.entry(prefix).or_default() += 1;
            res += Self::path_sum(left, prefix, sum, prefix_map);
            res += Self::path_sum(right, prefix, sum, prefix_map);
            *prefix_map.entry(prefix).or_default() -= 1;
            res
        } else {
            0
        }
    }
}

impl Solution {
    fn path_sum(root: TreeLink, sum: i32) -> i32 {
        let prefix_map: &mut HashMap<i32, i32> = &mut HashMap::new();
        prefix_map.insert(0, 1);
        root.path_sum(0, sum, prefix_map)
    }
}

#[test]
fn test() {
    let root = tree!(
        10,
        tree!(5, tree!(3, tree!(3), tree!(-2)), tree!(2, None, tree!(1))),
        tree!(-3, None, tree!(11))
    );
    assert_eq!(Solution::path_sum(root, 8), 3);
}

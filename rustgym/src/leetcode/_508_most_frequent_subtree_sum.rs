struct Solution;
use rustgym_util::*;
use std::collections::HashMap;

trait Postorder {
    fn postorder(&self, hs: &mut HashMap<i32, usize>, max: &mut usize) -> i32;
}

impl Postorder for TreeLink {
    fn postorder(&self, hs: &mut HashMap<i32, usize>, max: &mut usize) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.postorder(hs, max);
            let right = node.right.postorder(hs, max);
            let sum = val + left + right;
            let count = hs.entry(sum).or_default();
            *count += 1;
            *max = (*max).max(*count);
            sum
        } else {
            0
        }
    }
}

impl Solution {
    fn find_frequent_tree_sum(root: TreeLink) -> Vec<i32> {
        let mut hs: HashMap<i32, usize> = HashMap::new();
        let mut max: usize = 0;
        root.postorder(&mut hs, &mut max);
        hs.into_iter()
            .filter_map(|(k, v)| if v == max { Some(k) } else { None })
            .collect()
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(2), tree!(-3));
    let mut res = vec![2, -3, 4];
    let mut ans = Solution::find_frequent_tree_sum(root);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
    let root = tree!(5, tree!(2), tree!(-5));
    let mut res = vec![2];
    let mut ans = Solution::find_frequent_tree_sum(root);
    res.sort_unstable();
    ans.sort_unstable();
    assert_eq!(ans, res);
}

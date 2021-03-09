struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, max: i32, count: &mut usize);
}

impl Preorder for TreeLink {
    fn preorder(&self, max: i32, count: &mut usize) {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            if val >= max {
                *count += 1;
            }
            node.left.preorder(max.max(val), count);
            node.right.preorder(max.max(val), count);
        }
    }
}

impl Solution {
    fn good_nodes(root: TreeLink) -> i32 {
        let mut res = 0;
        root.preorder(std::i32::MIN, &mut res);
        res as i32
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(1, tree!(3), None), tree!(4, tree!(1), tree!(5)));
    let res = 4;
    assert_eq!(Solution::good_nodes(root), res);
    let root = tree!(3, tree!(3, tree!(4), tree!(2)), None);
    let res = 3;
    assert_eq!(Solution::good_nodes(root), res);
    let root = tree!(1);
    let res = 1;
    assert_eq!(Solution::good_nodes(root), res);
}

struct Solution;
use rustgym_util::*;

trait Preorder {
    fn preorder(&self, l: i32, r: i32, sum: &mut i32);
}

impl Preorder for TreeLink {
    fn preorder(&self, l: i32, r: i32, sum: &mut i32) {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let val = node.val;
            if val >= l && val <= r {
                *sum += val;
            }
            if val > l {
                left.preorder(l, r, sum)
            }
            if val < r {
                right.preorder(l, r, sum);
            }
        }
    }
}

impl Solution {
    fn range_sum_bst(root: TreeLink, l: i32, r: i32) -> i32 {
        let mut sum = 0;
        root.preorder(l, r, &mut sum);
        sum
    }
}

#[test]
fn test() {
    let root = tree!(10, tree!(5, tree!(3), tree!(7)), tree!(15, None, tree!(18)));
    assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
}

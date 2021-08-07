struct Solution;
use rustgym_util::*;

trait Closest {
    fn search(&self, target: f64) -> i32;
    fn preorder(link: &TreeLink, diff: &mut f64, res: &mut i32, target: f64);
}

impl Closest for TreeLink {
    fn search(&self, target: f64) -> i32 {
        let mut diff = std::f64::MAX;
        let mut res = 0;
        Self::preorder(self, &mut diff, &mut res, target);
        res
    }

    fn preorder(link: &TreeLink, diff: &mut f64, res: &mut i32, target: f64) {
        if let Some(node) = link {
            let node = node.borrow();
            let val = node.val as f64;
            let delta = (val - target).abs();
            if delta < *diff {
                *diff = delta;
                *res = node.val;
            }
            if target < val {
                Self::preorder(&node.left, diff, res, target);
            }
            if target > val {
                Self::preorder(&node.right, diff, res, target)
            }
        }
    }
}

impl Solution {
    fn closest_value(root: TreeLink, target: f64) -> i32 {
        root.search(target)
    }
}

#[test]
fn test() {
    let root = tree!(4, tree!(2, tree!(1), tree!(3)), tree!(5));
    assert_eq!(Solution::closest_value(root, 3.714_286), 4);
}

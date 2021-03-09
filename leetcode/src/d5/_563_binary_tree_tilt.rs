struct Solution;
use rustgym_util::*;

trait Tilt {
    fn find_tilt(&self, tilt: &mut i32) -> i32;
}

impl Tilt for TreeLink {
    fn find_tilt(&self, tilt: &mut i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let left_sum = left.find_tilt(tilt);
            let right_sum = right.find_tilt(tilt);
            *tilt += (left_sum - right_sum).abs();
            node.val + left_sum + right_sum
        } else {
            0
        }
    }
}

impl Solution {
    fn find_tilt(root: TreeLink) -> i32 {
        let mut tilt = 0;
        root.find_tilt(&mut tilt);
        tilt
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2), tree!(3));
    assert_eq!(Solution::find_tilt(root), 1);
}

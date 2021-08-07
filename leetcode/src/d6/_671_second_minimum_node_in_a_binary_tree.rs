struct Solution;
use rustgym_util::*;

trait SecondMinimum {
    fn find_second_minimum_value(&self, min: &mut Option<i32>) -> Option<i32>;
    fn option_min(a: Option<i32>, b: Option<i32>) -> Option<i32>;
}

impl SecondMinimum for TreeLink {
    fn find_second_minimum_value(&self, min: &mut Option<i32>) -> Option<i32> {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if min.is_none() {
                *min = Some(node.val);
            }
            if min.unwrap() == node.val {
                let min_left = Self::find_second_minimum_value(left, min);
                let min_right = Self::find_second_minimum_value(right, min);
                Self::option_min(min_left, min_right)
            } else {
                Some(node.val)
            }
        } else {
            None
        }
    }
    fn option_min(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(i32::min(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }
}

impl Solution {
    fn find_second_minimum_value(root: TreeLink) -> i32 {
        let mut min = None;
        if let Some(second_min) = root.find_second_minimum_value(&mut min) {
            second_min
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(2), tree!(5, tree!(5), tree!(7)));
    assert_eq!(Solution::find_second_minimum_value(root), 5);
    let root = tree!(2, tree!(2), tree!(2));
    assert_eq!(Solution::find_second_minimum_value(root), -1);
}

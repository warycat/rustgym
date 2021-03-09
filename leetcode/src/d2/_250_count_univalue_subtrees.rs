struct Solution;
use rustgym_util::*;

trait Postorder {
    fn postorder(&self, count: &mut i32) -> Option<i32>;
}

impl Postorder for TreeLink {
    fn postorder(&self, count: &mut i32) -> Option<i32> {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            match (left, right) {
                (Some(_), Some(_)) => {
                    if let (Some(left_val), Some(right_val)) =
                        (left.postorder(count), right.postorder(count))
                    {
                        if left_val == val && right_val == val {
                            *count += 1;
                            Some(val)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                (Some(_), None) => {
                    if let Some(left_val) = left.postorder(count) {
                        if left_val == val {
                            *count += 1;
                            Some(val)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                (None, Some(_)) => {
                    if let Some(right_val) = right.postorder(count) {
                        if right_val == val {
                            *count += 1;
                            Some(val)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                (None, None) => {
                    *count += 1;
                    Some(val)
                }
            }
        } else {
            None
        }
    }
}

impl Solution {
    fn count_unival_subtrees(root: TreeLink) -> i32 {
        let mut res = 0;
        root.postorder(&mut res);
        res
    }
}

#[test]
fn test() {
    let root = tree!(5, tree!(1, tree!(5), tree!(5)), tree!(5, None, tree!(5)));
    let res = 4;
    assert_eq!(Solution::count_unival_subtrees(root), res);
}

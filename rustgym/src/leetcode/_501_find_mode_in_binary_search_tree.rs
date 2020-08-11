struct Solution;
use rustgym_util::*;

trait Inorder {
    fn inorder(&self, prev: &mut Option<i32>, count: &mut usize, f: &mut impl FnMut(i32, usize));
}

impl Inorder for TreeLink {
    fn inorder(&self, prev: &mut Option<i32>, count: &mut usize, f: &mut impl FnMut(i32, usize)) {
        if let Some(node) = self {
            let node = node.borrow();
            Self::inorder(&node.left, prev, count, f);
            if let Some(prev_val) = prev.as_mut() {
                if *prev_val == node.val {
                    *count += 1;
                } else {
                    *count = 1;
                    *prev = Some(node.val);
                }
            } else {
                *prev = Some(node.val);
                *count = 1;
            }
            f(node.val, *count);
            Self::inorder(&node.right, prev, count, f);
        }
    }
}

impl Solution {
    fn find_mode(root: TreeLink) -> Vec<i32> {
        let mut max = 0;
        let mut count = 0;
        let mut prev: Option<i32> = None;
        let mut modes: Vec<i32> = vec![];
        root.inorder(&mut prev, &mut count, &mut |_, count| {
            max = usize::max(count, max);
        });
        prev = None;
        count = 0;
        root.inorder(&mut prev, &mut count, &mut |val, count| {
            if count == max {
                modes.push(val);
            }
        });
        modes
    }
}

#[test]
fn test() {
    let root = tree!(1, None, tree!(2, tree!(2), None));
    assert_eq!(Solution::find_mode(root), vec![2]);
}

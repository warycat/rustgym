struct Solution;
use rustgym_util::*;

trait Inorder {
    fn all_left(self) -> Vec<TreeLink>;
    fn all_right(self) -> Vec<TreeLink>;
}

impl Inorder for TreeLink {
    fn all_left(self) -> Vec<TreeLink> {
        let mut link = self;
        let mut stack = vec![];
        while let Some(node) = link {
            let left = node.borrow_mut().left.take();
            stack.push(Some(node));
            link = left;
        }
        stack
    }
    fn all_right(self) -> Vec<TreeLink> {
        let mut link = self;
        let mut stack = vec![];
        while let Some(node) = link {
            let right = node.borrow_mut().right.take();
            stack.push(Some(node));
            link = right;
        }
        stack
    }
}

struct TreeIter {
    stack: Vec<TreeLink>,
    forward: bool,
}

impl TreeIter {
    fn new(root: TreeLink, forward: bool) -> Self {
        let stack = if forward {
            root.all_left()
        } else {
            root.all_right()
        };
        TreeIter { stack, forward }
    }
}

impl Iterator for TreeIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Some(node)) = self.stack.pop() {
            let val = node.borrow().val;
            if self.forward {
                let right = node.borrow_mut().right.take();
                self.stack.append(&mut right.all_left());
            } else {
                let left = node.borrow_mut().left.take();
                self.stack.append(&mut left.all_right());
            }
            Some(val)
        } else {
            None
        }
    }
}

impl Solution {
    fn two_sum_bs_ts(root1: TreeLink, root2: TreeLink, target: i32) -> bool {
        use std::cmp::Ordering::*;
        let mut iter1 = TreeIter::new(root1, true);
        let mut iter2 = TreeIter::new(root2, false);
        let mut o1 = iter1.next();
        let mut o2 = iter2.next();
        while o1.is_some() && o2.is_some() {
            let x1 = o1.unwrap();
            let x2 = o2.unwrap();
            let sum = x1 + x2;
            match sum.cmp(&target) {
                Less => {
                    o1 = iter1.next();
                }
                Greater => {
                    o2 = iter2.next();
                }
                Equal => {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    let root1 = tree!(2, tree!(1), tree!(4));
    let root2 = tree!(1, tree!(0), tree!(3));
    let target = 5;
    let res = true;
    assert_eq!(Solution::two_sum_bs_ts(root1, root2, target), res);
    let root1 = tree!(0, tree!(-10), tree!(10));
    let root2 = tree!(5, tree!(1, tree!(0), tree!(2)), tree!(7));
    let target = 18;
    let res = false;
    assert_eq!(Solution::two_sum_bs_ts(root1, root2, target), res);
}

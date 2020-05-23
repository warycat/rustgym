struct Solution;
use std::collections::VecDeque;
use util::*;

trait Preorder {
    fn preorder(self, left_queue: &mut VecDeque<TreeLink>, right_queue: &mut VecDeque<TreeLink>);
}

impl Preorder for TreeLink {
    fn preorder(self, left_queue: &mut VecDeque<TreeLink>, right_queue: &mut VecDeque<TreeLink>) {
        if let Some(node) = self {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            left_queue.push_back(Some(node));
            right_queue.push_back(right);
            left.preorder(left_queue, right_queue);
        }
    }
}

impl Solution {
    fn upside_down_binary_tree(root: TreeLink) -> TreeLink {
        if root.is_some() {
            let mut left_queue: VecDeque<TreeLink> = VecDeque::new();
            let mut right_queue: VecDeque<TreeLink> = VecDeque::new();
            root.preorder(&mut left_queue, &mut right_queue);
            let mut root: TreeLink = left_queue.pop_front().unwrap();
            while let (Some(Some(left)), Some(right)) =
                (left_queue.pop_front(), right_queue.pop_front())
            {
                left.borrow_mut().right = root;
                left.borrow_mut().left = right;
                root = Some(left);
            }
            root
        } else {
            root
        }
    }
}

#[test]
fn test() {
    let root = tree!(1, tree!(2, tree!(4), tree!(5)), tree!(3));
    let res = tree!(4, tree!(5), tree!(2, tree!(3), tree!(1)));
    assert_eq!(Solution::upside_down_binary_tree(root), res);
}

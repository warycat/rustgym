struct Solution;
use rustgym_util::*;

use std::collections::VecDeque;

impl Solution {
    fn is_even_odd_tree(root: TreeLink) -> bool {
        let mut queue: VecDeque<TreeLink> = VecDeque::new();
        let mut level = 0;
        queue.push_back(root);
        while !queue.is_empty() {
            let n = queue.len();
            let mut nodes = vec![];
            for _ in 0..n {
                if let Some(node) = queue.pop_front().unwrap() {
                    let mut node = node.borrow_mut();
                    nodes.push(node.val);
                    queue.push_back(node.left.take());
                    queue.push_back(node.right.take());
                }
            }
            if nodes.iter().any(|&x| x % 2 == level % 2) {
                return false;
            }
            if nodes
                .windows(2)
                .any(|w| (level % 2 == 0 && w[0] >= w[1]) || (level % 2 == 1 && w[0] <= w[1]))
            {
                return false;
            }
            level += 1;
        }
        true
    }
}

#[test]
fn test() {
    let root = tree!(
        1,
        tree!(10, tree!(3, tree!(12), tree!(8)), None),
        tree!(4, tree!(7, tree!(6), None), tree!(9, None, tree!(2)))
    );
    let res = true;
    assert_eq!(Solution::is_even_odd_tree(root), res);
    let root = tree!(5, tree!(9, tree!(3), tree!(5)), tree!(1, tree!(7), None));
    let res = false;
    assert_eq!(Solution::is_even_odd_tree(root), res);
    let root = tree!(5, tree!(4, tree!(3), tree!(3)), tree!(2, tree!(7), None));
    let res = false;
    assert_eq!(Solution::is_even_odd_tree(root), res);
}

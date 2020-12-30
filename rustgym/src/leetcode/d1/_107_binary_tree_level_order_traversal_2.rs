struct Solution;
use rustgym_util::*;
use std::collections::VecDeque;

struct Pair {
    tree: TreeLink,
    level: usize,
}

impl Solution {
    fn level_order_bottom(root: TreeLink) -> Vec<Vec<i32>> {
        let mut levels: Vec<Vec<i32>> = vec![];
        let mut queue: VecDeque<Pair> = VecDeque::new();
        if root.is_some() {
            queue.push_back(Pair {
                tree: root,
                level: 0,
            });
            while let Some(pair) = queue.pop_front() {
                let node = pair.tree.unwrap();
                let mut node = node.borrow_mut();
                let level = pair.level;
                let val = node.val;
                let left = node.left.take();
                let right = node.right.take();
                if let Some(group) = levels.get_mut(level) {
                    group.push(val);
                } else {
                    let group = vec![val];
                    levels.insert(level, group);
                }
                if left.is_some() {
                    queue.push_back(Pair {
                        tree: left,
                        level: level + 1,
                    });
                }
                if right.is_some() {
                    queue.push_back(Pair {
                        tree: right,
                        level: level + 1,
                    });
                }
            }
        }
        levels.reverse();
        levels
    }
}

#[test]
fn test() {
    let tree = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let nodes = vec![vec![15, 7], vec![9, 20], vec![3]];
    assert_eq!(Solution::level_order_bottom(tree), nodes);
}
